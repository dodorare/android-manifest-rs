use crate::common::XmlAttribute;
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_namespaces_definition(attributes: &XmlAttribute) -> TokenStream {
    attributes
        .namespaces
        .iter()
        .map(|(prefix, namespace)| {
            if attributes.default_namespace.as_deref().eq(&Some(prefix)) {
                quote!(
                  .default_ns(#namespace)
                )
            } else {
                quote!(
                  .ns(#prefix, #namespace)
                )
            }
        })
        .collect()
}
