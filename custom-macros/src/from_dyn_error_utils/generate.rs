use proc_macro::TokenStream;
use quote::quote;

use crate::from_dyn_error_utils::parse::{FromDynErrorParsedAst, FromDynErrorParsedAstVariant};

pub fn generate<'a>(data: FromDynErrorParsedAst<'a>) -> TokenStream {
	let name = data.enum_ident;
	let variant_name = data.variant_ident;
	match data.variant {
		FromDynErrorParsedAstVariant::Named(ident) => {
			let code = quote! {
				impl<T: crate::error::ToBoxedError> From<T> for #name {
					fn from(value: T) -> Self {
						Self::#variant_name { #ident: value.into_dyn_error() }
					}
				}
			};

			code.into()
		},
		FromDynErrorParsedAstVariant::Unnamed => {
			let code = quote! {
				impl<T: crate::error::ToBoxedError> From<T> for #name {
					fn from(value: T) -> Self {
						Self::#variant_name(value.into_dyn_error())
					}
				}
			};

			code.into()
		},
		FromDynErrorParsedAstVariant::Unit => {
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
}
