use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, Item, ItemEnum, ItemStruct, Result, parse_macro_input, spanned::Spanned};

pub fn derive_syntax(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    let derived = match item {
        Item::Enum(item) => derive_syntax_for_enum(item),
        Item::Struct(item) => derive_syntax_for_struct(item),
        _ => {
            return Error::new(
                item.span(),
                "proc_macro `Syntax` can only derive `struct` or `enum`.",
            )
            .into_compile_error()
            .into();
        }
    };

    match derived {
        Ok(token_stream) => token_stream.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn derive_syntax_for_enum(_item: ItemEnum) -> Result<proc_macro2::TokenStream> {
    Ok(quote! {})
}

fn derive_syntax_for_struct(_item: ItemStruct) -> Result<proc_macro2::TokenStream> {
    Ok(quote! {})
}
