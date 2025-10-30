mod keyword;
mod syntax;
mod token;
mod tuple;

/// Derive `Syntax` trait for tuples (T,...)
#[proc_macro]
pub fn derive_tuple_syntax(args: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tuple::derive_tuple_syntax(args)
}

/// Derive `Syntax` trait for `struct`s / `enum`s.
#[proc_macro_derive(Syntax, attributes(syntax, fatal, from, map_err, try_filter))]
pub fn derive_syntax(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syntax::derive_syntax(input)
}

/// Derive a `keyword` sytax.
#[proc_macro]
pub fn keyword(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    keyword::derive_keyword(item)
}

/// Derive a `token` sytax.
#[proc_macro]
pub fn token(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    token::derive_token(item)
}
