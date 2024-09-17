use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(debug))]
struct AutoDebugInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), FieldsInfo>,
}
#[derive(Debug, FromField)]
#[darling(attributes(debug))]
struct FieldsInfo {
    ident: Option<syn::Ident>,
    #[darling(default)]
    skip: bool,
}

pub(crate) fn process_auto_debug(input: DeriveInput) -> TokenStream {
    let AutoDebugInfo {
        ident,
        generics,
        data: darling::ast::Data::Struct(fields),
    } = AutoDebugInfo::from_derive_input(&input)
        .expect("AutoDebug can only be derived for structs")
    else {
        panic!("AutoDebug can only be derived for structs")
    };

    let fields = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let skip = field.skip;
        if skip {
            // quote! {
            //     .field(stringify!(#ident), &"...")
            // }
            quote! {
                // .field(stringify!(#ident), &"...")
            }
        } else {
            quote! {
                .field(stringify!(#ident), &self.#ident)
            }
        }
    });
    println!("fields +++++++{:#?}", fields);
    quote! {
        impl std::fmt::Debug for #ident #generics{
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!(#ident))
                    #(#fields)*
                    .finish()

            }
        }
    }
}
