use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;

use crate::axum_response::parse::{AxumResponseParsedAst, AxumResponseStatus};

pub fn generate<'a>(data: AxumResponseParsedAst<'a>) -> TokenStream {
	let name = data.enum_name;

	let variants_code = data.variants.into_iter().map(|v| {
		let status = match v.status {
			AxumResponseStatus::Integer(s) => {
				let literal = Literal::u16_unsuffixed(s);

				quote! {
					*response.status_mut() = unsafe { StatusCode::from_u16(#literal).unwrap_unchecked() };
				}
			},
			AxumResponseStatus::Ident(ident) => {
				quote! {
					*response.status_mut() = axum::http::status::StatusCode::#ident;
				}
			},
		};

		let variant_name = v.variant_name;
		let variant_param = v.body.map(|_| quote! { (a) });
		let variant_body = match v.body {
			Some(_) => {
				quote! {
					let mut response = a.into_response();
					#status
					response
				}
			},
			None => {
				quote! {
					let mut response = axum::response::Response::new(axum::body::Body::empty());
					#status
					response
				}
			},
		};

		quote! {
			Self::#variant_name #variant_param => {
				#variant_body
			}
		}
	});

	let code = quote! {
		impl axum::response::IntoResponse for #name {
			fn into_response(self) -> axum::response::Response {
				match self {
					#(#variants_code),*
				}
			}
		}
	};

	code.into()
}
