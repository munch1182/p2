// default_with.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Field, Fields, Lit, parse_macro_input,
};

pub(crate) fn derive_default_with_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let pubs = &input.vis;

    if let Data::Struct(s) = input.data
        && let Fields::Named(fields) = s.fields
    {
        let fields = fields.named;

        let new_params = fields
            .iter()
            .filter(|f| is_default_with(&f.attrs))
            .map(|field| {
                let name = &field.ident;
                let ty = &field.ty;
                quote! { #name: #ty }
            });

        // 生成new函数中的字段初始化
        let new_initializations = fields.iter().filter_map(get_default_with_path);

        let expanded = quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                #[inline]
                #pubs fn default_with(#(#new_params),*) -> Self {
                    Self {
                        #(#new_initializations,)*
                    }
                }
            }
        };

        return expanded.into();
    };

    quote!().into()
}

fn is_default_with(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("default_with")
            && attr
                .parse_args::<syn::Ident>()
                .map(|ident| ident == "no_default")
                .unwrap_or(false)
    })
}

fn get_default_with_path(field: &Field) -> Option<proc_macro2::TokenStream> {
    if let Some(name) = &field.ident {
        if let Some(attr) = field
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("default_with"))
        {
            if let Ok(Lit::Str(str)) = attr.parse_args::<syn::Lit>() {
                let str = str.value();
                if let Ok(str) = syn::parse_str::<syn::ExprPath>(&str) {
                    return Some(quote! { #name: #str() });
                }
            } else if attr
                .parse_args::<syn::Ident>()
                .map(|ident| ident == "no_default")
                .unwrap_or(true)
            {
                return Some(quote! { #name });
            } else {
                return None;
            }
        } else {
            return Some(quote! { #name: Default::default() });
        }
    }
    None
}