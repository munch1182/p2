use proc_macro::TokenStream;
use quote::quote;

use crate::parse::Args;
mod parse;

///
/// 设置`libcommon::prelude::logsetup`和`libcommon::prelude::log_setup_with_writer`
///
/// 当设置`log_setup_with_writer`时，会在函数结束时调用`log_flush`
/// 
/// 传入`level`的参数，会调用`libcommon::log::log_set_level`设置日志级别
///
/// # example
///
/// #### 默认实现，不使用`logfile`
///
/// ```ignore
/// #[logsetup]
/// fn main() {
/// }
/// ```
/// #### 配置参数并将日志写入文件
/// featrues = `[logfile]`
/// ```ignore
/// #[logsetup(&task, "logdir")]
/// fn main() {
/// }
/// ```
/// #### 使用默认task实现
/// 默认使用tokio异步，因此需要其运行时
/// feature = `[logfile_default]`
/// ```ignore
/// #[logsetup("logdir")]
/// #[tokio::main]
/// async fn main() {
/// }
/// ```
/// #### 设置最大level
/// ```ignore
/// #[logsetup("info")]
/// #[tokio::main]
/// async fn main() {
/// }
/// ```
#[proc_macro_attribute]
pub fn logsetup(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(attr as Args);
    let input_fn = syn::parse_macro_input!(item as syn::ItemFn);
    logsetup_impl(input_fn, args)
}

fn logsetup_impl(input_fn: syn::ItemFn, args: Args) -> TokenStream {
    // 提取函数的信息
    let fn_vis = &input_fn.vis; // 可见性
    let fn_sig = &input_fn.sig; // 函数签名
    //let fn_name = &fn_sig.ident; // 函数名
    let fn_block = &input_fn.block; // 函数体
    let is_async_fn = fn_sig.asyncness.is_some(); // 是否是异步函数
    let fn_attrs: Vec<_> = input_fn
        .attrs
        .iter()
        .filter(|a| !a.path().is_ident("logsetup"))
        .collect();

    let execute_block = if is_async_fn {
        quote! { (|| async move #fn_block)().await }
    } else {
        quote! { (|| #fn_block)() }
    };

    let log_setup = {
        let task = match args.task {
            Some(task) => quote! { #task },
            None => quote! { &libcommon::log::LogWriterDefaultTask },
        };
        let set_level = match args.level {
            Some(level) => quote! { libcommon::log::log_set_level(#level) },
            None => quote! {},
        };
        match args.dir {
            Some(dir) => {
                quote! {
                    libcommon::log::log_setup_with_writer(#task, #dir);
                    #set_level;
                    let result = #execute_block;
                    libcommon::log::log_flush();
                    result
                }
            }
            None => {
                quote! {
                    libcommon::log::log_setup();
                    #set_level;
                    #execute_block
                }
            }
        }
    };
    quote! {
       #(#fn_attrs)*
       #fn_vis #fn_sig {
           #log_setup
       }
    }
    .into()
}
