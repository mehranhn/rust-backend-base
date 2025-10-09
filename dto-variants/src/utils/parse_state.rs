use std::collections::HashMap;

use syn::{DeriveInput, Error, Field, Ident, spanned::Spanned};

use crate::state::State;

pub fn parse_state(input: DeriveInput) -> Result<State, syn::Error> {
	let fields = match input.data {
		syn::Data::Struct(data_struct) => match data_struct.fields {
			syn::Fields::Named(fields_named) => fields_named
				.named
				.into_iter()
				.map(|f| (f.ident.clone().unwrap(), f))
				.collect::<HashMap<Ident, Field>>(),
			syn::Fields::Unnamed(fields_unnamed) => fields_unnamed
				.unnamed
				.into_iter()
				.enumerate()
				.map(|(i, f)| (Ident::new(format!("{i}").as_str(), f.span()), f))
				.collect::<HashMap<Ident, Field>>(),
			syn::Fields::Unit => {
				return Err(Error::new(
					data_struct.struct_token.span, "Unit structs are not supported",
				));
			},
		},
		syn::Data::Enum(data_enum) => {
			return Err(Error::new(
				data_enum.enum_token.span, "Enums are not supported yet",
			));
		},
		syn::Data::Union(data_union) => {
			return Err(Error::new(
				data_union.union_token.span, "Union are not supported",
			));
		},
	};

	let attrs = input
		.attrs
		.into_iter()
		.filter(|a| {
			let path = &a.path();
			!(path.is_ident("variant_include") || path.is_ident("variant_exclude"))
		})
		.collect();

	Ok(State::new(attrs, input.vis, input.generics, fields, vec![]))
}
