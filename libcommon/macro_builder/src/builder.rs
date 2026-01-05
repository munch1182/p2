// builder.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, Ident, parse_macro_input,
};

pub(crate) fn derive_builder_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let builder_name = Ident::new(&format!("{name}Builder"), name.span());
    let pubs = input.vis;
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    if let Data::Struct(s) = input.data
        && let Fields::Named(fields) = s.fields
    {
        let fields = fields.named;

        // 分离必选、可选字段
        let (required_fields, optional_fields): (Vec<_>, Vec<_>) =
            fields.iter().partition(|f| !is_option_type(&f.ty));

        // 生成new函数参数
        let new_params = required_fields.iter().map(|field| {
            let name = &field.ident;
            let ty = &field.ty;
            quote! { #name: #ty }
        });

        // 生成setter方法
        let setter_methods = optional_fields.iter().map(|field| {
            let name = &field.ident;
            let inner_type = get_opt_type(&field.ty);
            quote! {
                #[inline]
                #pubs fn #name(mut self, #name: #inner_type) -> Self {
                    self.#name = Some(#name);
                    self
                }
            }
        });

        // 生成builder结构体字段
        let builder_fields = fields.iter().map(|f| {
            let name = &f.ident;
            let ty = &f.ty;
            quote! { #name: #ty }
        });

        // 生成new函数中的字段初始化
        let new_initializations = fields.iter().map(|field| {
            let field_name = &field.ident;
            if is_option_type(&field.ty) {
                quote! { #field_name: None }
            } else {
                quote! { #field_name }
            }
        });

        // 生成build方法中的字段映射
        let build_mappings = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! { #field_name: self.#field_name }
        });

        let expanded = quote! {
            #pubs struct #builder_name #ty_generics #where_clause {
                #(#builder_fields,)*
            }

            impl #impl_generics #builder_name #ty_generics #where_clause {
                #[inline]
                #pubs fn new(#(#new_params),*) -> Self {
                    Self {
                        #(#new_initializations,)*
                    }
                }

                #(#setter_methods)*

                #[inline]
                #pubs fn build(self) -> #name #ty_generics #where_clause{
                    #name {
                        #(#build_mappings,)*
                    }
                }
            }
        };

        return expanded.into();
    };

    quote!().into()
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