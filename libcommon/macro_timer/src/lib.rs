use proc_macro::TokenStream;
use quote::quote;

/**
 * 对函数进行计时
 * 通过日志输出函数执行时间
 */
#[proc_macro_attribute]
pub fn timer(_attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_timer_code(item, false, "timer")
}

/**
 * 只在debug模式下对函数进行计时
 * 在release模式下会被完全移除
 */
#[proc_macro_attribute]
pub fn timer_debug(_attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_timer_code(item, true, "timer_debug")
}

fn generate_timer_code(item: TokenStream, debug_only: bool, prefix: &str) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_vis = &input.vis;
    let fn_sig = &input.sig;
    let fn_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| !a.path().is_ident("timer") && !a.path().is_ident("timer_debug"))
        .collect();
    let is_async_fn = input.sig.asyncness.is_some();

    let timer_code = if is_async_fn {
        quote! {
            use std::time::Instant;
            let start = Instant::now();
            let result = (|| async move #fn_block)().await;
            let duration = start.elapsed();
            info!("{}: async fn({}) took {}ms.", #prefix, stringify!(#fn_name), duration.as_millis());
            result
        }
    } else {
        quote! {
            use std::time::Instant;
            let start = Instant::now();
            let result = (||#fn_block)();
            let duration = start.elapsed();
            info!("{}: fn({}) took {}ms.", #prefix, stringify!(#fn_name), duration.as_millis());
            result
        }
    };

    let output = if debug_only {
        quote! {
            #(#fn_attrs)*
            #fn_vis #fn_sig {
                #[cfg(debug_assertions)]
                {
                    #timer_code
                }
                #[cfg(not(debug_assertions))]
                {
                    #fn_block
                }
            }
        }
    } else {
        quote! {
            #(#fn_attrs)*
            #fn_vis #fn_sig {
                #timer_code
            }
        }
    };

    output.into()
}
