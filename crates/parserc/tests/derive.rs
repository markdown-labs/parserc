use parserc::{
    ControlFlow, Input, Kind, ParseError, Parser,
    lang::{LangInput, TokenStream},
    syntax::{Syntax, keyword},
    take_while,
};

#[allow(unused)]
#[derive(Clone, Debug)]
struct MockError;

impl From<(ControlFlow, Kind)> for MockError {
    fn from(_: (ControlFlow, Kind)) -> Self {
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
}

#[derive(Syntax)]
#[syntax(input = I)]
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
}
