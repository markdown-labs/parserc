use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{
    Attribute, Error, Fields, Item, ItemEnum, ItemStruct, Result, Type, parse::Parser,
    parse_macro_input, spanned::Spanned,
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
    ty_input: Type,
}

impl Default for Syntax {
    fn default() -> Self {
        Self {
            ty_input: syn::parse2(quote! { I }).unwrap(),
        }
    }
}

fn parse_syntax_options(attrs: &[Attribute]) -> Result<Syntax> {
    let Some(syntax) = attrs.iter().find(|attr| attr.path().is_ident("syntax")) else {
        return Ok(Default::default());
    };

    let meta_list = match &syntax.meta {
        syn::Meta::Path(path) => {
            return Err(Error::new(path.span(), "Empty body, expect `syntax(...)`"));
        }
        syn::Meta::List(meta_list) => meta_list,
        syn::Meta::NameValue(value) => return Err(Error::new(value.span(), "Unsupport syntax.")),
    };

    let mut ty_input: Option<Type> = None;

    let parser = syn::meta::parser(|meta| {
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
        } else {
            error!("Unsupport macro `syntax` option `{}`.", ident);
        }

        Ok(())
    });

    parser.parse2(meta_list.tokens.to_token_stream())?;

    if let Some(ty_input) = ty_input {
        Ok(Syntax { ty_input })
    } else {
        Ok(Default::default())
    }
}

fn derive_syntax_for_enum(item: ItemEnum) -> Result<proc_macro2::TokenStream> {
    let Syntax { ty_input } = parse_syntax_options(&item.attrs)?;

    let ident = &item.ident;
    let ident_str = ident.to_string();

    let (impl_generic, type_generic, where_clause) = item.generics.split_for_impl();

    let (fields, to_spans): (Vec<_>, Vec<_>) = item
        .variants
        .iter()
        .map(|varint| {
            let variant_ident = &varint.ident;

            let parse_fields = varint
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

            let to_spans = varint
                .fields
                .members()
                .map(|member| match member {
                    syn::Member::Named(ident) => {
                        quote! {
                           #ident.to_span()
                        }
                    }
                    syn::Member::Unnamed(index) => {
                        let ident = format_ident!("ident_{}", index);
                        quote! {
                            #ident.to_span()
                        }
                    }
                })
                .collect::<Vec<_>>();

            let parse = if let Fields::Named(_) = &varint.fields {
                quote! {
                    Ok(#ident::#variant_ident {
                        #(#parse_fields),*
                    })
                }
            } else {
                quote! {
                    Ok(#ident::#variant_ident(#(#parse_fields),*))
                }
            };

            let field_idents = varint
                .fields
                .members()
                .map(|member| match member {
                    syn::Member::Named(ident) => ident,
                    syn::Member::Unnamed(index) => format_ident!("ident_{}", index),
                })
                .collect::<Vec<_>>();

            let match_arm = if let Fields::Named(_) = &varint.fields {
                quote! { Self::#variant_ident { #(#field_idents),* } }
            } else {
                quote! { Self::#variant_ident ( #(#field_idents),* ) }
            };

            let parse = quote! {
                let parser = | input: &mut #ty_input | {
                        use parserc::syntax::InputSyntaxExt;
                        #parse
                };

                if let Some(value) = parser.ok().parse(input)? {
                    return Ok(value);
                }
            };

            let to_span = quote! {
                #match_arm => {
                    let mut lhs = parserc::Span::None;
                    #(
                        lhs = lhs.union(&#to_spans);
                    )*

                    lhs
                }
            };

            (parse, to_span)
        })
        .unzip();

    Ok(quote! {
        impl #impl_generic parserc::syntax::Syntax<#ty_input> for #ident #type_generic #where_clause {
            #[inline]
            fn parse(input: &mut #ty_input) -> Result<Self, <#ty_input as parserc::Input>::Error> {
                use parserc::syntax::InputSyntaxExt;
                use parserc::Parser;
                #(#fields)*

                Err(parserc::Kind::Syntax(#ident_str,parserc::ControlFlow::Recovable,input.to_span()).into())
            }

            #[inline]
            fn to_span(&self) -> parserc::Span {
                match self {
                    #(#to_spans),*
                }
            }
        }
    })
}

fn derive_syntax_for_struct(item: ItemStruct) -> Result<proc_macro2::TokenStream> {
    let Syntax { ty_input } = parse_syntax_options(&item.attrs)?;

    let ident = &item.ident;

    let (impl_generic, type_generic, where_clause) = item.generics.split_for_impl();

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

    let to_spans = item
        .fields
        .members()
        .map(|member| match member {
            syn::Member::Named(ident) => {
                quote! {
                   self.#ident.to_span()
                }
            }
            syn::Member::Unnamed(index) => {
                quote! {
                    self.#index.to_span()
                }
            }
        })
        .collect::<Vec<_>>();

    let parse = if item.semi_token.is_some() {
        quote! {
            Ok(Self(#(#parse_fields),*))
        }
    } else {
        quote! {
            Ok(Self {
                #(#parse_fields),*
            })
        }
    };

    Ok(quote! {
        impl #impl_generic parserc::syntax::Syntax<#ty_input> for #ident #type_generic #where_clause {
            #[inline]
            fn parse(input: &mut #ty_input) -> Result<Self, <#ty_input as parserc::Input>::Error> {
                use parserc::syntax::InputSyntaxExt;
                #parse
            }

            #[inline]
            fn to_span(&self) -> parserc::Span {
                let mut lhs = parserc::Span::None;
                #(
                    lhs = lhs.union(&#to_spans);
                )*

                lhs
            }
        }
    })
}
