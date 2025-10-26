use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, LitInt, parse_macro_input};

pub fn derive_tuple_syntax(args: TokenStream) -> TokenStream {
    let len = parse_macro_input!(args as LitInt);

    let len = match len.base10_parse::<usize>() {
        Ok(num) => {
            if num < 3 {
                return Error::new(len.span(), "length argument must greater than 2.")
                    .into_compile_error()
                    .into();
            }

            num
        }
        Err(err) => return err.into_compile_error().into(),
    };

    let mut stmts = vec![];
    for i in 2..len {
        let mut types = vec![];

        let mut pos = vec![];

        for j in 0..i {
            types.push(
                format!("T{}", j)
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap(),
            );

            pos.push(
                format!("self.{}", j)
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap(),
            );
        }

        stmts.push(quote! {
            impl<I,E, #(#types),*> Syntax<I,E> for (#(#types),*)
            where
                I: Input,
                E: ParseError<I>,
                #(#types: Syntax<I,E>),*
            {
                fn parse(input: I) -> Result<Self, I, E> {
                    #(
                        let (#types,input) = #types::parse(input)?;
                    )*

                    Ok(((#(#types),*),input))
                }
            }
        });
    }

    quote! {
        #(#stmts)*
    }
    .into()
}
