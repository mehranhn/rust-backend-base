mod generate;
mod parse;

use proc_macro::TokenStream;

pub fn sortable(input: TokenStream) -> TokenStream {
	let ast: syn::DeriveInput = syn::parse(input).unwrap();

	match parse::parse(&ast) {
		Ok(data) => generate::generate(data),
		Err(err) => err.into_compile_error().into(),
	}
}
