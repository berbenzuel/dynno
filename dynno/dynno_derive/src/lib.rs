use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, parse_macro_input, DeriveInput};
use log::debug;
// use dynno_core::model::DbModel;

#[proc_macro_derive(Collection)]
pub fn derive_collection(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => &fields.named,
            _ => panic!("Named fields only"),
        },
        _ => panic!("Structs only"),
    };

    quote! {
        impl #name {
            pub fn write_definition(&self, writer: &mut impl std::io::Write) -> std::io::Result<()>{
                Ok(())
            }
        }
    }.into()

    // let fields = match &input.data {
    //     syn::Data::Struct(data) => match &data.fields {
    //         syn::Fields::Named(fields) => &fields.named,
    //         _ => panic!("Named fields only"),
    //     },
    //     _ => panic!("Structs only"),
    // };
}
