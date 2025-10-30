use quote::quote;
use syn::{Error, ExprClosure, Ident, Token, parse::Parse, parse_macro_input, spanned::Spanned};

struct Keyword {
    ident: Ident,
    #[allow(unused)]
    comma: Token![,],
    value: ExprClosure,
}

impl Parse for Keyword {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;

        Ok(Self {
            ident,
            comma: input.parse()?,
            value: input.parse()?,
        })
    }
}

pub fn derive_token(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Keyword {
        ident,
        comma: _,
        value,
    } = parse_macro_input!(item as Keyword);

    if value.inputs.len() != 1 {
        return Error::new(
            value.inputs.span(),
            "Only support closure with signature `|item: I| { ... }`",
        )
        .into_compile_error()
        .into();
    }

    let param = value.inputs.first().unwrap();

    let syn::Pat::Type(pat_type) = param else {
        return Error::new(
            value.inputs.span(),
            "Only support closure with signature `|item: I| { ... }`",
        )
        .into_compile_error()
        .into();
    };

    let ty = &pat_type.ty;

    quote! {
        /// Token `#ident`
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct #ident<I>(pub I);

        impl<I> parserc::syntax::Syntax<I> for #ident<I>
        where
            I: parserc::Input<Item = #ty>,
        {
            #[inline]
            fn parse(input: &mut I) -> Result<Self, I::Error> {
                use parserc::Parser;
                parserc::take_while(#value).parse(input).map(|input| Self(input))
            }

            #[inline]
            fn to_span(&self) -> parserc::Span {
                self.0.to_span()
            }
        }

    }
    .into()
}
