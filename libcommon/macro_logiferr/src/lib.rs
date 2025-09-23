use proc_macro::TokenStream;
use quote::quote;

///
/// 如果函数返回错误，则打印错误日志
///
/// 需要函数返回[Result]类型
#[proc_macro_attribute]
pub fn logiferr(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_vis = &input.vis;
    let fn_sig = &input.sig;
    let fn_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| !a.path().is_ident("logiferr"))
        .collect();

    let output = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig{
            let result:Result<_> = (||#fn_block)();
            if let Err(e) = &result {
                error!("fn({}) failed: {}", stringify!(#fn_name), e);
            }
            result
        }
    };
    output.into()
}
