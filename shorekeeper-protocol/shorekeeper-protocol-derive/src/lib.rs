use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Meta, MetaList};

#[proc_macro_derive(MessageID, attributes(message_id))]
pub fn message_id_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let id = match input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("message_id"))
    {
        Some(attr) => match attr.meta {
            Meta::List(MetaList { ref tokens, .. }) => tokens.into_token_stream(),
            _ => panic!("Invalid message_id attribute value"),
        },
        _ => 0u16.into_token_stream(),
    };

    TokenStream::from(quote! {
        impl crate::MessageID for #struct_name {
            const MESSAGE_ID: u16 = #id;
        }
    })
}
