mod from_dyn_error_utils;

use proc_macro::TokenStream;

#[proc_macro_derive(FromBoxError, attributes(dyn_error))]
pub fn from_dyn_error(input: TokenStream) -> TokenStream {
    from_dyn_error_utils::from_dyn_error(input)
}
