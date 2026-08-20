#![recursion_limit = "256"]

mod common;
mod de;
mod ser;

use proc_macro::TokenStream;

#[proc_macro_derive(XmlDeserialize, attributes(xml))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    de::expand_derive_deserialize(&ast)
        .unwrap_or_else(|message| syn::Error::new_spanned(&ast, message).to_compile_error())
        .into()
}

#[proc_macro_derive(XmlSerialize, attributes(xml))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    ser::expand_derive_serialize(&ast)
        .unwrap_or_else(|message| syn::Error::new_spanned(&ast, message).to_compile_error())
        .into()
}
