use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Field, Fields, Ident, Lit, Type, parse_macro_input,
    spanned::Spanned,
};

/// 生成的`Builder`结构和其对应的`new`方法、`build`方法以及[Option]字段的`setter`方法
///
///
/// 如果该字段不是[Option]类型，则认为是必选字段，需要在`new`方法中传入；
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

/// 为属性生成`with`方法用以替换
///
/// # usage
/// 当字段类型为[Option]时，默认生成的`with`的传入参数为其泛型
/// 如果字段被标记为`#[with(keep)]`，则生成的with方法保留为[Option]类型
/// 如果字段被标记为`#[with(skip)]`，则不生成该字段的`with`方法
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

/// 为属性生成`default_with`方法用生成对象
///
///
/// 如果该属性没有实现[Default]，需要手动标记`#[default_with(no_default)]`并在生成的`default_with`方法中手动传入
/// 如果该属性有无参的生成方法，可以手动标记该方法`#[default_with("Struct::fn")]`生成默认值
///
/// # usage
///```ignore
/// #[derive(Default_With)]
/// struct Data {
///     #[default_with(no_default)]
///     no_default: NoDefaultStruct,
///     #[default_with("DefaultStruct::new")]
///     default: DefaultStruct,
/// }
///
/// struct NoDefaultStruct(u8);
/// struct DefaultStruct(u8);
///
/// impl DefaultStruct {
///     fn new() -> Self {
///         Self(1)
///     }
/// }
///
/// let user = Data::default_with(NoDefaultStruct(1));
/// assert!(user.no_default.0 == 1);
/// assert!(user.default.0 == 1);
/// ```
#[proc_macro_derive(Default_With, attributes(default_with))]
pub fn derive_default_with(input: TokenStream) -> TokenStream {
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
        let new_initializations = fields.iter().map(get_default_with_path);

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

fn is_skipped_with(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("with")
            && attr
                .parse_args::<syn::Ident>()
                .map(|ident| ident == "skip")
                .unwrap_or(false)
    })
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
    if let Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
        && segment.ident == "Option"
        && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
        && let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first()
    {
        return inner_ty.clone();
    }
    ty.clone()
}
