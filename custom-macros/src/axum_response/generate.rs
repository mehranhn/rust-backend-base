use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;

use crate::axum_response::parse::{AxumResponseParsedAst, AxumResponseStatus};

pub fn generate<'a>(data: AxumResponseParsedAst<'a>) -> TokenStream {
	let name = data.enum_name;

	let variants_code = data.variants.into_iter().map(|v| {
		let status = match v.status {
			AxumResponseStatus::Integer(s) => {
				let literal = Literal::u16_suffixed(s);

				quote! {
					unsafe { axum::http::status::StatusCode::from_u16(#literal).unwrap_unchecked() }
				}
			},
			AxumResponseStatus::Ident(ident) => {
				quote! {
					axum::http::status::StatusCode::#ident
				}
			},
		};

		let variant_name = v.variant_name;
		let variant_param = v.body.map(|_| quote! { (a) });
		let variant_body = match v.body {
			Some((_, is_json)) => {
				if is_json {
					quote! {
						let mut response = axum::Json(a).into_response();
						*response.status_mut() = #status;
						response
					}
				} else {
					quote! {
						let mut response = a.into_response();
						*response.status_mut() = #status;
						response
					}
				}
			},
			None => {
				quote! {
					let mut response = axum::response::Response::new(axum::body::Body::empty());
					*response.status_mut() = #status;
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
