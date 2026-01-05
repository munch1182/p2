use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, Ident, parse_macro_input, spanned::Spanned};

pub(crate) fn with_builder_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    if let Data::Struct(s) = input.data
        && let Fields::Named(fields) = s.fields
    {
        let fields = fields.named;

        // 生成setter方法，跳过有#[with(skip)]的字段
        let setter_methods = fields.iter().filter_map(|field| {
            let name = &field.ident;
            if !is_skipped_with(&field.attrs) && name.is_some() {
                let _type = &field.ty;
                let fn_str = format!("with_{}", name.as_ref().unwrap());
                let fn_name = Ident::new(&fn_str, name.span());
                // 如果字段类型为Option且没有被标记为keep，则生成Option泛型的setter方法
                if is_option_type(_type) && !is_keep_with(&field.attrs) {
                    let _type = get_opt_type(_type);
                    Some(quote! {
                        #[inline]
                        pub fn #fn_name(mut self, #name: #_type) -> Self {
                            self.#name = Some(#name);
                            self
                        }
                    })
                } else {
                    Some(quote! {
                        #[inline]
                        pub fn #fn_name(mut self, #name: #_type) -> Self {
                            self.#name = #name;
                            self
                        }
                    })
                }
            } else {
                None
            }
        });

        let expanded = quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                #(#setter_methods)*
            }
        };

        return expanded.into();
    };

    quote!().into()
}

fn is_skipped_with(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("with")
            && attr
                .parse_args::<syn::Ident>()
                .map(|ident| ident == "skip")
                .unwrap_or(false)
    })
}

fn is_keep_with(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("with")
            && attr
                .parse_args::<syn::Ident>()
                .map(|ident| ident == "keep")
                .unwrap_or(false)
    })
}

// 检查类型是否为Option
fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        type_path
            .path
            .segments
            .last()
            .map(|segment| segment.ident == "Option")
            .unwrap_or(false)
    } else {
        false
    }
}

// 获取Option内部的类型
fn get_opt_type(ty: &syn::Type) -> syn::Type {
    if let syn::Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
        && segment.ident == "Option"
        && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
        && let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first()
    {
        return inner_ty.clone();
    }
    ty.clone()
}
