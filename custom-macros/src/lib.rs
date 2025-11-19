mod from_dyn_error_utils;
mod from_sqlx_error;
mod axum_response;

use proc_macro::TokenStream;

#[proc_macro_derive(FromBoxError, attributes(dyn_error))]
pub fn from_dyn_error(input: TokenStream) -> TokenStream {
	from_dyn_error_utils::from_dyn_error(input)
}

#[proc_macro_derive(FromSqlxError, attributes(esqlx, no_boxed))]
pub fn from_sqlx_error(input: TokenStream) -> TokenStream {
    from_sqlx_error::from_sqlx_error(input)
}

#[proc_macro_derive(AxumResponse, attributes(response))]
pub fn axum_response(input: TokenStream) -> TokenStream {
    axum_response::axum_response(input)
}
