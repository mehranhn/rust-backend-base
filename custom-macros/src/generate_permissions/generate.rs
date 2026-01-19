use proc_macro::TokenStream;
use quote::{format_ident, quote};

use crate::generate_permissions::parse::GeneratePermissionsParsedAst;

pub fn generate<'a>(data: GeneratePermissionsParsedAst<'a>) -> TokenStream {
	let variants = data.variants_names.into_iter().map(|v| {
		let ident = if let Some(s) = data.prefix {
			format_ident!("{}{}", s.value(), v)
		} else {
			v.clone()
		};

		quote! {
			pub struct #ident;

			impl crate::permission::Permission for #ident {
				fn permission() -> crate::permission::Permissions {
					crate::permission::Permissions::#v
				}
			}
		}
	});

	quote! {
		#(#variants)*
	}.into()
}
