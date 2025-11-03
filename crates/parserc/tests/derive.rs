use parserc::{
    Input, Kind, ParseError, Parser,
    lang::{LangInput, TokenStream},
    syntax::{Syntax, keyword},
    take_while,
};
use parserc_derive::token;

#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq)]
struct MockError;

impl From<Kind> for MockError {
    fn from(_: Kind) -> Self {
        Self
    }
}

impl ParseError for MockError {
    fn control_flow(&self) -> parserc::ControlFlow {
        todo!()
    }

    fn into_fatal(self) -> Self {
        todo!()
    }

    fn span(&self) -> parserc::Span {
        todo!()
    }
}

struct _Ident<I>(I);

impl<I> Syntax<I> for _Ident<I>
where
    I: LangInput,
{
    fn parse(input: &mut I) -> Result<Self, <I as Input>::Error> {
        let ident = take_while(|c: u8| c.is_ascii()).parse(input)?;

        Ok(Self(ident))
    }

    fn to_span(&self) -> parserc::Span {
        self.0.to_span()
    }
}

#[derive(Syntax)]
#[syntax(input = I, map_err = |err| err)]
struct _Ident2<I>(_Ident<I>)
where
    I: LangInput;

#[allow(unused)]
#[derive(Syntax)]
#[syntax(input = TokenStream<'a, MockError>)]
struct _Iden3<'a> {
    ident: _Ident<TokenStream<'a, MockError>>,
}

keyword!(KeywordFn, "fn");
keyword!(class);

#[derive(Syntax)]
#[syntax(input = I)]
enum _Key<I>
where
    I: LangInput,
{
    Fn(KeywordFn<I>),
    Class(Class<I>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let mut input = TokenStream::from("hello world");
        _Iden3::parse(&mut input).unwrap();

        let mut input: TokenStream<'_, MockError> = TokenStream::from("fn");
        KeywordFn::parse(&mut input).unwrap();
        let mut input: TokenStream<'_, MockError> = TokenStream::from("class");
        Class::parse(&mut input).unwrap();
    }

    #[test]
    fn test_token() {
        token!(Variable, |c: u8| { c.is_ascii_alphabetic() });

        let mut input: TokenStream<'_, MockError> = TokenStream::from("fn");

        assert_eq!(
            Variable::parse(&mut input),
            Ok(Variable(TokenStream::from("fn")))
        );
    }
}
