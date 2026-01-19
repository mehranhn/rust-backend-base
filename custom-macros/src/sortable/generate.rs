use heck::ToPascalCase;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::sortable::parse::{SortableParsedAst, SortableParsedField};

fn get_iden(field: &SortableParsedField<'_>) -> Ident {
	if let Some(ref s) = field.rename_to {
		format_ident!("{}", s.value())
	} else {
		format_ident!("{}", field.ident.to_string().to_pascal_case())
	}
}

fn get_default_variant<'a>(data: &SortableParsedAst<'a>) -> Ident {
	for field in data.variants_names.iter() {
		if field.is_default {
			return get_iden(field);
		}
	}

	get_iden(&data.variants_names[0])
}

pub fn generate<'a>(data: SortableParsedAst<'a>) -> TokenStream {
	let name = format_ident!("{}SortColumns", data.struct_name);

	let default_variant = get_default_variant(&data);
	let variants = data.variants_names.into_iter().map(|field| {
		let ident = get_iden(&field);

		quote! {
			#ident
		}
	});

	quote! {
		#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, utoipa::ToSchema)]
		pub enum #name {
			#(#variants),*
		}

		impl Default for #name {
			fn default() -> Self {
				Self::#default_variant
			}
		}
	}
	.into()
}
