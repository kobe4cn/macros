use proc_macro::TokenStream;
use quote::quote;

//proc macro crate
//for enum, we'd like to generate from impls for each variant
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    //get indent of the enum
    let ident = input.ident;
    //get generic parameters of the enum
    let generics = input.generics;
    //get variants of the enum
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => {
            panic!("EnumFrom can only be derived for enums")
        }
    };
    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        let fields = match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields
                        .unnamed
                        .first()
                        .expect("Expected exactly one field in the variant");
                    let ty = &field.ty;
                    quote! {
                        impl #generics From<#ty> for #ident #generics{
                            fn from(variant: #ty)->Self{
                                #ident::#var(variant)
                            }
                        }
                    }
                }
            }

            syn::Fields::Unit => quote! {},
            syn::Fields::Named(_fields) => quote! {},
        };
        fields
    });

    quote! {
        #(#from_impls)*
    }
    .into()
    // println!("{:#?}", input);
}
