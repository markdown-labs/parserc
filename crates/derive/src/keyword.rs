use convert_case::{Case, Casing};
use quote::{ToTokens, format_ident, quote};
use syn::{Error, Ident, LitStr, Token, parse::Parse, parse_macro_input};

struct Keyword {
    ident: Ident,
    #[allow(unused)]
    comma: Option<Token![,]>,
    value: Option<LitStr>,
}

impl Parse for Keyword {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse().map_err(|err| {
            Error::new(
                err.span(),
                r#"Create operator syntax using syntax `keyword!(ident -> "xxx")`"#,
            )
        })?;

        let comma: Option<Token![,]> = input.parse()?;

        let keyword = if comma.is_some() {
            let keyword: LitStr = input.parse()?;
            Some(keyword)
        } else {
            None
        };

        Ok(Self {
            ident,
            comma,
            value: keyword,
        })
    }
}

pub fn derive_keyword(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let keyword = parse_macro_input!(item as Keyword);

    let value = keyword.ident.to_string();

    let ident = format_ident!("{}", value.to_case(Case::UpperCamel));

    let value = if let Some(value) = keyword.value {
        value.to_token_stream()
    } else {
        syn::parse2(quote! { #value }).unwrap()
    };

    quote! {
        /// Keyword `#ident`
        pub struct #ident<I>(I) where I: parserc::lang::LangInput;

        impl<I> parserc::syntax::Syntax<I> for #ident<I>
        where
            I: parserc::lang::LangInput,
        {
            fn parse(input: &mut I) -> Result<Self, I::Error> {
                parserc::keyword(#value).parse(input).map(|input| Self(input))
            }
        }

    }
    .into()
}
