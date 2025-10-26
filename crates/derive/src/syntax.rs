use proc_macro::TokenStream;
use syn::{Item, parse_macro_input};

pub(crate) fn derive_syntax(input: TokenStream) -> TokenStream {
    let _item = parse_macro_input!(input as Item);
    todo!()
}
