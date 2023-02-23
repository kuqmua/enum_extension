#![deny(
    clippy::indexing_slicing,
    // clippy::integer_arithmetic,//detects inside quote!{}
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

///you need to install this crates to use this macro(check on work was only with this versions)
/// strum = "0.24.1"
/// strum_macros = "0.24.3"
/// convert_case = "0.6.0"
#[proc_macro_derive(EnumExtension)]
pub fn derive_enum_extension(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //it only supported for enums without values
    let ast: syn::DeriveInput = syn::parse(input).expect("EnumExtension syn::parse(input) failed");
    //todo to implement into_array() and into_vec - must implement Default for all inner variant types
    let len = match ast.data.clone() {
        syn::Data::Enum(enum_item) => enum_item.variants.len(),
        _ => panic!("EnumVariantCount only works on Enums"),
    };
    let variants = match ast.data {
        syn::Data::Enum(enum_item) => enum_item.variants.into_iter().map(|v| {
            let variant_ident = v.ident;
            match v.fields {
                syn::Fields::Named(fields_named) => {
                    let generated = fields_named.named.into_iter().map(|field| {
                        let field_ident = field.ident;
                        quote::quote! { #field_ident: Default::default() }
                    });
                    quote::quote! {
                       #variant_ident {
                           #(#generated),*
                       }
                    }
                }
                syn::Fields::Unnamed(_) => quote::quote! { #variant_ident(Default::default()) },
                syn::Fields::Unit => quote::quote! { #variant_ident },
            }
        }),
        _ => panic!("EnumIntoArray works only on enums"),
    };
    let name = &ast.ident;
    let gen = quote::quote! {
        impl #name {
            pub fn get_length() -> usize {
                #len
            }
            pub fn into_array() -> [#name; #len] {
                [ #(#name::#variants),* ]
            }
            pub fn into_vec() -> Vec<Self> {
                use strum::IntoEnumIterator;
                let mut self_vec = Vec::with_capacity(Self::get_length());
                for self_variant in Self::iter() {
                    self_vec.push(self_variant);
                }
                self_vec
            }
            pub fn into_string_name_and_variant_hashmap() -> std::collections::HashMap<String, Self> {
                use strum::IntoEnumIterator;
                let mut variants_hashmap: std::collections::HashMap<String, Self> =
                    std::collections::HashMap::with_capacity(Self::get_length());
                for variant in Self::iter() {
                    variants_hashmap.insert(format!("{}", variant), variant);
                }
                variants_hashmap
            }
            pub fn into_string_name_and_variant_tuple_vec() -> Vec<(String, Self)> {
                use strum::IntoEnumIterator;
                let mut variants_vec = Vec::with_capacity(Self::get_length());
                for variant in Self::iter() {
                    variants_vec.push((format!("{}", variant), variant));
                }
                variants_vec
            }
            pub fn to_upper_snake_case(&self) -> String {
                use convert_case::Casing;
                use convert_case;
                format!("{:?}", self).to_case(convert_case::Case::Snake).to_uppercase()
            }
            pub fn to_lower_snake_case(&self) -> String {
                use convert_case::Casing;
                use convert_case;
                format!("{:?}", self).to_case(convert_case::Case::Snake).to_lowercase()
            }
        }
    };
    gen.into()
}
