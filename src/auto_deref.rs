use std::vec;

use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(deref))]
struct AutoDerefInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), FieldsInfo>,
    #[darling(default)]
    mutable: bool,
    #[darling(default)]
    field: Option<syn::Ident>,
}

#[derive(Debug, FromField)]
struct FieldsInfo {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

pub(crate) fn process_auto_deref(input: DeriveInput) -> TokenStream {
    let AutoDerefInfo {
        ident,
        generics,
        data: darling::ast::Data::Struct(fields),
        mutable,
        field,
    } = AutoDerefInfo::from_derive_input(&input)
        .expect("AutoDeref can only be derived for structs")
    else {
        panic!("AutoDeref can only be derived for structs")
    };

    let (fd, ty) = if let Some(field) = field {
        match fields.iter().find(|f| f.ident.as_ref().unwrap() == &field) {
            Some(f) => (field, &f.ty),
            None => panic!("Field {} not found in struct", field),
        }
    } else if fields.len() == 1 {
        let field = fields.iter().next().unwrap();
        (field.ident.as_ref().unwrap().clone(), &field.ty)
    } else {
        panic!("AutoDeref can only be derived for structs with exactly one field");
    };
    let mut code = vec![quote! {
        impl #generics std::ops::Deref for #ident #generics{
            type Target = #ty;
            fn deref(&self)->&Self::Target{
                &self.#fd
            }
        }
    }];
    if mutable {
        code.push(quote! {
            impl #generics std::ops::DerefMut for #ident #generics{
                fn deref_mut(&mut self)->&mut Self::Target{
                    &mut self.#fd
                }
            }
        });
    }

    quote! {
        #(#code)*
    }
}
