use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

pub(crate) fn derive_getter_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let struct_vis = input.vis;
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    if let Data::Struct(s) = input.data
        && let Fields::Named(fields) = s.fields
    {
        let fields = fields.named;
        let getter = fields.iter().filter_map(|field| {
            let field_name = field.ident.as_ref()?;
            let field_vis = &field.vis;

            if has_skip_getter(field) || !is_visibility_less_than_struct(&struct_vis, field_vis) {
                return None;
            }

            let field_type = &field.ty;
            let method_name = format_ident!("get_{}", field_name);
            if is_simple_value(field_type) {
                Some(quote! {
                    #struct_vis fn #method_name(&self) -> #field_type {
                        self.#field_name
                    }
                })
            } else {
                Some(quote! {
                    #struct_vis fn #method_name(&self) -> &#field_type {
                        &self.#field_name
                    }
                })
            }
        });

        return quote! {
            impl #impl_generics #struct_name #ty_generics #where_clause {
                #(#getter)*
            }
        }
        .into();
    }

    quote! {}.into()
}

fn is_simple_value(field: &Type) -> bool {
    let simple_types = [
        "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
        "f32", "f64", "bool", "char", "()",
    ];
    let type_str = quote!(#field).to_string();
    return simple_types.contains(&type_str.as_str());
}

fn is_visibility_less_than_struct(
    struct_vis: &syn::Visibility,
    field_vis: &syn::Visibility,
) -> bool {
    visibility_level(field_vis) < visibility_level(struct_vis)
}

fn has_skip_getter(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| {
        attr.path().is_ident("getter")
            && attr
                .parse_args::<syn::Ident>()
                .map(|ident| ident == "skip")
                .unwrap_or(false)
    })
}

fn visibility_level(vis: &syn::Visibility) -> u8 {
    match vis {
        syn::Visibility::Public(_) => 10,
        syn::Visibility::Restricted(restricted) => {
            let path = &restricted.path;
            if path.is_ident("crate") {
                5
            } else if path.is_ident("super") {
                4
            } else {
                3
            }
        }
        syn::Visibility::Inherited => 0,
    }
}
