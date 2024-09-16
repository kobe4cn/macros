use darling::{ast::Data, FromDeriveInput, FromField, FromVariant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(FromDeriveInput)]
struct EnumFromDarling {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<EnumVariants, ()>,
}

#[derive(FromVariant)]
struct EnumVariants {
    ident: syn::Ident,
    fields: darling::ast::Fields<EnumVariantFields>,
}
#[derive(FromField)]
struct EnumVariantFields {
    ty: syn::Type,
}

pub(crate) fn process_enum_from_darling(input: DeriveInput) -> TokenStream {
    let EnumFromDarling {
        ident,
        generics,
        data: Data::Enum(data),
    } = EnumFromDarling::from_derive_input(&input).unwrap()
    else {
        panic!("EnumFromDarling can only be derived for enums")
    };

    let from_impls = data.iter().map(|variant| {
        let var = &variant.ident;
        let style = &variant.fields.style;
        match style {
            darling::ast::Style::Tuple if variant.fields.len() == 1 => {
                let field = variant
                    .fields
                    .iter()
                    .next()
                    .expect("Expected exactly one field in the variant");
                let ty = &field.ty;
                quote! {
                    impl #generics From<#ty> for #ident #generics{
                        fn from(variant:#ty)->Self{
                            #ident::#var(variant)
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });
    quote! {
        #(#from_impls)*
    }
}
