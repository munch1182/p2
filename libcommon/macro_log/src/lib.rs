use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;

///
/// 设置[log_setup](libcommon::log::log_setup)和[log_setup_with_writer](libcommon::log::log_setup_with_writer)
///
/// 当设置log_setup_with_writer时，会在函数结束时调用[log_flush](libcommon::log::log_flush)
///
/// 为避免其它影响，该属性应该放在最前面，除非其它属性需要放在前面
///
/// # example
///
/// #### 默认实现，不使用logfile
///
/// ```ignore
/// #[logsetup]
/// fn main() {
/// }
/// ```
/// #### 配置参数并将日志写入文件
/// featrues = [logfile]
/// ```ignore
/// #[logsetup(&task, "logdir")]
/// fn main() {
/// }
/// ```
/// #### 使用默认task实现
/// 默认使用tokio异步，因此需要其运行时
/// feature = [logfile_default]
/// ```ignore
/// #[logsetup("logdir")]
/// #[tokio::main]
/// async fn main() {
/// }
/// ```
#[proc_macro_attribute]
pub fn logsetup(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let args = syn::parse_macro_input!(attr as Args);
    let fn_block = &input.block;
    let fn_vis = &input.vis;
    let fn_sig = &input.sig;
    let fn_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| !a.path().is_ident("logsetup"))
        .collect();
    let is_async_fn = fn_sig.asyncness.is_some();

    let quote = {
        let setup_log = |task: Option<Expr>, dir: Expr| {
            if let Some(task) = task {
                quote! { libcommon::log::log_setup_with_writer(#task, #dir); }
            } else {
                quote! { libcommon::log::log_setup_with_writer(&libcommon::log::LogWriterDefaultTask, #dir); }
            }
        };

        let execute_block = if is_async_fn {
            quote! { (|| async move #fn_block)().await }
        } else {
            quote! { (|| #fn_block)() }
        };

        match (args.task, args.dir) {
            (task, Some(dir)) => {
                let log_code = setup_log(task, dir);
                quote! {
                    #(#fn_attrs)*
                    #fn_vis #fn_sig {
                        #log_code;
                        let result = #execute_block;
                        libcommon::log::log_flush();
                        result
                    }
                }
            }
            _ => {
                quote! {
                    #(#fn_attrs)*
                    #fn_vis #fn_sig {
                        libcommon::log::log_setup();
                        #fn_block
                    }
                }
            }
        }
    };

    quote.into()
}

#[derive(Debug)]
struct Args {
    task: Option<Expr>,
    dir: Option<Expr>,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut p1 = None;
        let mut p2 = None;
        if !input.is_empty() {
            p1 = Some(input.parse()?);
            if !input.is_empty() {
                let _: syn::Token![,] = input.parse()?;
                p2 = Some(input.parse()?);
            }
        }
        // 如果只有一个参数，则认为是dir
        let (task, dir) = if p2 == None && p1 != None {
            (None, p1)
        } else {
            (p1, p2)
        };
        Ok(Args { task, dir })
    }
}
