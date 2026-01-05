use syn::{Expr, Ident, LitStr, Token, spanned::Spanned};

#[derive(Debug)]
pub(crate) struct Args {
    pub(crate) task: Option<Expr>,
    pub(crate) dir: Option<LitStr>,
    pub(crate) level: Option<String>,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut task = None;
        let mut dir = None;
        let mut level = None;

        while !input.is_empty() {
            if input.peek(Ident) && input.peek2(Token![=]) {
                // 尝试解析为键值对
                let key = input.parse::<Ident>()?;
                let _eq: Token![=] = input.parse()?;
                let value: Expr = input.parse()?;

                match key.to_string().as_str() {
                    "task" => {
                        task = Some(expr_to_ident(value)?);
                    }
                    "dir" => {
                        dir = Some(expr_to_lit_str(&value)?);
                    }
                    "level" => {
                        level = Some(expr_to_level(&value)?);
                    }
                    _ => {}
                }
            } else {
                let expr: Expr = input.parse()?;
                match expr {
                    syn::Expr::Reference(_) => {
                        task = Some(expr_to_ident(expr)?);
                    }
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(lit_str),
                        ..
                    }) => {
                        let value = lit_str.value();
                        let (is_level, _) = is_level_str(&value);
                        if is_level {
                            level = Some(value);
                        } else {
                            dir = Some(lit_str);
                        }
                    }
                    _ => {}
                }
            }

            // 如果不是最后一个参数，解析逗号
            if !input.is_empty() {
                let _comma: Token![,] = input.parse()?;
            }
        }

        Ok(Args { task, dir, level })
    }
}

// 辅助函数：将表达式转换为Ident
fn expr_to_ident(expr: syn::Expr) -> syn::Result<Expr> {
    if !matches!(expr, syn::Expr::Reference(_)) {
        return Err(syn::Error::new(
            expr.span(),
            "task must be a reference to a struct",
        ));
    }

    Ok(expr)
}

// 辅助函数：将表达式转换为LitStr
fn expr_to_lit_str(expr: &syn::Expr) -> syn::Result<LitStr> {
    match expr {
        syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
            syn::Lit::Str(lit_str) => return Ok(lit_str.clone()),
            _ => {}
        },
        _ => {}
    }
    return Err(syn::Error::new(expr.span(), "dir must be a string literal"));
}

fn is_level_str(str: &str) -> (bool, Vec<&str>) {
    let levels = vec!["trace", "debug", "info", "warn", "error", "record"];
    return (levels.contains(&str), levels);
}

// 辅助函数：将表达式转换为level字符串
fn expr_to_level(expr: &syn::Expr) -> syn::Result<String> {
    let lit_str = expr_to_lit_str(expr)?;
    let value = lit_str.value();
    let (is_level, levels) = is_level_str(&value);

    if is_level {
        Ok(value)
    } else {
        Err(syn::Error::new(
            lit_str.span(),
            format!("level must be one of {}", levels.join(",")),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> std::io::Result<()> {
        let str = [
            r#""#,
            r#"task = &task, dir ="/a", level = "debug""#,
            r#"&task, "debug""#,
            r#""/a", "info""#,
            r#""/a""#,
        ];

        for ele in str {
            let args = syn::parse_str::<Args>(ele);
            println!("({ele}) ===> {args:?}");
        }
        Ok(())
    }

    struct _TaskA;
}
