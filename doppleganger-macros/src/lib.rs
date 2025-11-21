use doppleganger_macros_parse::{AdtDecl, Cons, EndOfStream, Struct};
use proc_macro2::TokenStream;
use unsynn::*;

#[proc_macro_derive(Doppleganger, attributes(dg))]
pub fn facet_macros(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dg_macros(input.into()).into()
}

fn dg_macros(input: TokenStream) -> TokenStream {
    let mut i = input.to_token_iter();

    // Parse as TypeDecl
    match i.parse::<Cons<AdtDecl, EndOfStream>>() {
        Ok(it) => match it.first {
            AdtDecl::Struct(parsed) => process_struct(parsed),
            AdtDecl::Enum(parsed) => todo!("Not yet implemented"),
        },
        Err(err) => {
            panic!("Could not parse type declaration: {err}");
        }
    }
}

fn process_struct(s: Struct) -> TokenStream {}
