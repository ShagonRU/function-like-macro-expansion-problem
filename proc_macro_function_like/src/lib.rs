extern crate proc_macro;
extern crate quote;

use proc_macro::{TokenStream};
use proc_macro2 as pm2;

use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, Ident, Token};
use quote::ToTokens;

// pg_query!(User, &mut executor, login=user_login, email=email)

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

            let tmp_s = m.to_token_stream().to_string();
            let mut strings = vec![];
            for s in tmp_s.split('=') {
                strings.push(s);
            }
            let column_name = Expr::Verbatim(strings.get(0).unwrap().parse()?);
            let value = Expr::Verbatim(strings.get(1).unwrap().parse()?);

            params.push((column_name, value, counter));
            counter += 1;
        }
        Ok(SimplePgQuery { model, executor, params, })
    }
}

#[proc_macro]
pub fn pg_query(input: TokenStream) -> TokenStream {
    let SimplePgQuery { model, executor, params} = parse_macro_input!(input);

    let table_name = quote::quote!{#model :: TABLE_NAME};
    let select_from_clause = quote::quote!(
        "SELECT * FROM ", #table_name,
    );

    let mut add_c = pm2::TokenStream::new();
    let mut and_clause = pm2::TokenStream::new();

    let arguments_count = params.len();

    for (field_alias, field_variable, arg_number_at_query) in params {

        let num_lit = pm2::Literal::usize_unsuffixed(arg_number_at_query);

        and_clause = quote::quote!(#and_clause #field_alias = $#num_lit);
        if arg_number_at_query != arguments_count {
            and_clause = quote::quote!(#and_clause  AND);
        }

        add_c = quote::quote!(#add_c .add_c(#field_variable));
    }

    let lit_and = pm2::Literal::string(&and_clause.to_string());
    let with_brackets = pm2::Group::new(
        pm2::Delimiter::Parenthesis,
        quote::quote!(#select_from_clause #lit_and)
    );

    let finalized_clause = quote::quote!(
        concatcp! #with_brackets
    );

    let expanded = quote::quote!({
        use crate::helper_traits::ChainedArguments as _;

        sqlx::query_as_with::<_, #model, _>(
            #finalized_clause,
            sqlx::postgres::PgArguments::default() #add_c,
        )
            .fetch_optional(#executor).await
    });

    TokenStream::from(expanded)
}
