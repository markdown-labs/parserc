use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Attribute, Error, Ident, Item, ItemEnum, ItemStruct, Path, Result, WhereClause, meta,
    parse::Parser, parse_macro_input, spanned::Spanned, token::Token,
};

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

struct Syntax {
    ty_input: proc_macro2::TokenStream,
    ty_error: proc_macro2::TokenStream,
    where_cause: proc_macro2::TokenStream,
}

fn parse_syntax_options(attrs: &[Attribute]) -> Result<Syntax> {
    let syntax = attrs.iter().find(|attr| attr.path().is_ident("syntax"));

    let Some(syntax) = syntax else { todo!() };

    let meta_list = match &syntax.meta {
        syn::Meta::Path(path) => {
            return Err(Error::new(path.span(), "Empty body, expect `syntax(...)`"));
        }
        syn::Meta::List(meta_list) => meta_list,
        syn::Meta::NameValue(value) => return Err(Error::new(value.span(), "Unsupport syntax.")),
    };

    let mut ty_input: Option<Path> = None;
    let mut ty_error: Option<Path> = None;

    let parser = meta::parser(|meta| {
        macro_rules! error {
            ($($t:tt)+) => {
                return Err(meta.error(format_args!($($t)+)))
            };
        }

        let Some(ident) = meta.path.get_ident() else {
            error!("Unsupport macro `syntax` option.");
        };

        if ident == "input" {
            ty_input = Some(meta.value()?.parse()?);
        } else if ident == "error" {
            ty_error = Some(meta.value()?.parse()?);
        } else {
            error!("Unsupport macro `syntax` option `{}`.", ident);
        }

        Ok(())
    });

    parser.parse(meta_list.tokens.to_token_stream().into())?;

    todo!()
}

fn derive_syntax_for_enum(_item: ItemEnum) -> Result<proc_macro2::TokenStream> {
    parse_syntax_options(&_item.attrs)?;

    Ok(quote! {})
}

fn derive_syntax_for_struct(item: ItemStruct) -> Result<proc_macro2::TokenStream> {
    parse_syntax_options(&item.attrs)?;

    let parse_fields = item
        .fields
        .members()
        .map(|member| match member {
            syn::Member::Named(ident) => {
                quote! {
                    #ident: input.parse()?
                }
            }
            syn::Member::Unnamed(_) => {
                quote! {input.parse()?}
            }
        })
        .collect::<Vec<_>>();

    Ok(quote! {})
}
