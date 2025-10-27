use parserc::lang::TokenStream;
use parserc_derive::Syntax;

#[allow(unused)]
#[derive(Syntax)]
#[syntax(input = I)]
struct Ident<I>(I);

#[allow(unused)]
#[derive(Syntax)]
#[syntax(input = TokenStream<'a>)]
struct Iden2<'a>(TokenStream<'a>);
