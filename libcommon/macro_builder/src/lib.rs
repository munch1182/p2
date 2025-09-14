use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, Ident, Type, parse_macro_input, spanned::Spanned};

/// 生成的Builder结构和其对应的new方法、build方法以及Option字段的setter方法
///
///
/// 如果该字段不是Option类型，则认为是必选字段，需要在new方法中传入；
/// 否则为其生成同名设置方法
///
/// # example
///```ignore
/// #[derive(Builder)]
/// struct User {
///    name: String,
///    age: u16,
///    agent: Option<bool>,
///    address: Option<String>,
/// }
///
/// let user = UserBuilder::new("Jack".to_string(), 22)
///    .address("Beijing".to_string())
///    .build();
/// assert!(user.address.is_some());
/// ```
#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let builder_name = Ident::new(&format!("{name}Builder"), name.span());
    let pubs = input.vis;

    if let Data::Struct(s) = input.data {
        if let Fields::Named(fields) = s.fields {
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

            // 组合所有的字段
            let fields = required_fields
                .iter()
                .chain(optional_fields.iter())
                .collect::<Vec<_>>();

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
                #pubs struct #builder_name {
                    #(#builder_fields,)*
                }

                impl #builder_name {
                    #[inline]
                    #pubs fn new(#(#new_params),*) -> Self {
                        Self {
                            #(#new_initializations,)*
                        }
                    }

                    #(#setter_methods)*

                    #[inline]
                    #pubs fn build(self) -> #name {
                        #name {
                            #(#build_mappings,)*
                        }
                    }
                }
            };

            return expanded.into();
        }
    };

    quote!().into()
}

/// 为属性生成with方法用以替换
/// <br>
/// 当字段类型为Option时，默认生成的with的传入参数为其泛型
/// <br>
/// 如果字段被标记为#[with(keep)]，则生成的with方法保留为Option类型
/// <br>
/// 如果字段被标记为#[with(skip)]，则不生成该字段的with方法
///
/// # example
///```ignore
/// #[derive(With, Default)]
/// struct User {
///    name: String,
///    #[with(skip)]
///    age: u16,
///    #[with(keep)]
///    agent: Option<bool>,
///    address: Option<String>,
/// }
///
/// let user = User::default().with_agent(None).with_address("Beijing".to_string());
/// assert!(user.agent.is_none());
/// ```
#[proc_macro_derive(With, attributes(with))]
pub fn with_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    if let Data::Struct(s) = input.data {
        if let Fields::Named(fields) = s.fields {
            let fields = fields.named;

            // 生成setter方法，跳过有#[with(skip)]的字段
            let setter_methods = fields.iter().filter_map(|field| {
                let name = &field.ident;
                if is_skipped_with(&field.attrs) {
                    None
                } else {
                    let _type = &field.ty;
                    let fn_name =
                        syn::Ident::new(&format!("with_{}", name.as_ref().unwrap()), name.span());
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
                }
            });

            let expanded = quote! {
                impl #name {
                    #(#setter_methods)*
                }
            };

            return expanded.into();
        }
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
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
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
fn get_opt_type(ty: &Type) -> Type {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return inner_ty.clone();
                    }
                }
            }
        }
    }
    ty.clone()
}
