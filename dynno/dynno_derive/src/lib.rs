use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, parse_macro_input, DeriveInput};
// use dynno_core::model::DbModel;

#[proc_macro_derive(DbModel)]
pub fn derive_db_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    quote! {
        impl #name {
            pub fn hi() {
                println!("hello");
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
