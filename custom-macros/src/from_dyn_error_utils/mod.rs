use proc_macro::TokenStream;
use quote::quote;

mod generate;
mod parse;

pub fn from_dyn_error(input: TokenStream) -> TokenStream {
	let ast: syn::DeriveInput = syn::parse(input).unwrap();

	match parse::parse(&ast) {
		Ok(parsed) => {
			let name = parsed.enum_ident;
			let variant_name = parsed.variant_ident;
			match parsed.variant {
				parse::FromDynErrorParsedAstVariant::Named(ident) => {
					let code = quote! {
						impl<T: crate::error::ToBoxedError> From<T> for #name {
							fn from(value: T) -> Self {
								Self::#variant_name { #ident: Box::new(value) }
							}
						}
					};

					code.into()
				},
				parse::FromDynErrorParsedAstVariant::Unnamed => {
					let code = quote! {
						impl<T: crate::error::ToBoxedError> From<T> for #name {
							fn from(value: T) -> Self {
								Self::#variant_name(Box::new(value))
							}
						}
					};

					code.into()
				},
				parse::FromDynErrorParsedAstVariant::Unit => {
					let code = quote! {
						impl<T: crate::error::ToBoxedError> From<T> for #name {
							fn from(value: T) -> Self {
								Self::#variant_name
							}
						}
					};

					code.into()
				},
			}
		},
		Err(err) => err.into_compile_error().into(),
	}
}
