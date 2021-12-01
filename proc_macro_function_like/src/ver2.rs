#![feature(box_into_inner)]
extern crate proc_macro;
extern crate quote;


use proc_macro::{TokenStream};
use proc_macro2 as pm2;

use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, Ident, Token};
use quote::ToTokens;


struct SimplePgQuery {
    model: Ident,
    executor: syn::ExprReference,
    params: Vec<(Expr, Expr, usize)>,
}

impl Parse for SimplePgQuery {
    fn parse(input: ParseStream) -> Result<Self> {
        let model: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let executor: syn::ExprReference = match input.parse::<Expr>()? {
            Expr::Reference(r) => {r},
            _ => {panic!("There should be '&mut executor', where executor - pool or connection.")}

        };
        input.parse::<Token![,]>()?;

        let mut params = vec![];
        let mut counter = 1;
        for m in Punctuated::<Expr, Token![,]>::parse_terminated(input)?.iter() {

            if let Expr::Binary(e) = m {
                params.push((Box::into_inner(e.left.clone()), Box::into_inner(e.right.clone()), counter));
            } else {
                // eprintln!("{:?}", &m);
                return Err(syn::Error::new_spanned(
                    m, format!("Only binary expressions like `col = value` allowed here! {:?}", &m).as_str()
                ))
            }
            counter += 1;
        }
        Ok(SimplePgQuery { model, executor, params, })
    }
}

macro_rules! format_literal {
    ($($arg:tt)*) => {{
        let value = format!($($arg)*);
        ::quote::quote!(#value)
    }}
}

pub fn pg_query_broken(input: TokenStream) -> TokenStream {
    let SimplePgQuery { model, executor, params} = parse_macro_input!(input);

    let mut add_c = pm2::TokenStream::new();
    let mut and_clause = pm2::TokenStream::new();

    let arguments_count = params.len();
    for (field_alias, field_variable, arg_number_at_query) in params {

        let dollar_with_num_str = format_literal!("${}", arg_number_at_query);

        and_clause = quote::quote!(#and_clause #field_alias = #dollar_with_num_str);
        if arg_number_at_query != arguments_count {
            and_clause = quote::quote!(#and_clause AND);
        }

        add_c = quote::quote!(#add_c .add_c(#field_variable));
    }

    let vectorized = and_clause.to_string()
        .split('"')
        .map(|x| pm2::Literal::string(x))
        .collect::<Vec<_>>();

    let expanded = quote::quote!({
        use crate::helper_traits::ChainedArguments as _;

        const __SELECT_CLAUSE: &str = concatcp!("SELECT * FROM ", #model::TABLE_NAME, " WHERE ",  #( #vectorized, )*);
        eprintln!("{:?}", __SELECT_CLAUSE);
        sqlx::query_as_with::<_, #model, _>(
            __SELECT_CLAUSE,
            sqlx::postgres::PgArguments::default() #add_c,
        )
            .fetch_optional(#executor).await
    });

    eprintln!("{}", &expanded);

    TokenStream::from(expanded)
}
