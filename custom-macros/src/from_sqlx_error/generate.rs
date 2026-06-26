use proc_macro2::Literal;
use quote::quote;
use syn::Ident;

use crate::from_sqlx_error::parse::{
	CustomError, CustomErrorVariant, FromSqlxParsedAst, Generic, GenericVariant,
};

fn generate_generic(data: &Generic<'_>) -> proc_macro2::TokenStream {
	match &data.variant {
		GenericVariant::Unit => {
			let ident = data.enum_variant_name;

			quote! {
				Self::#ident
			}
		},
		GenericVariant::Struct {
			field_name,
			is_boxed,
		} => {
			if *is_boxed {
				let ident = data.enum_variant_name;

				quote! {
					Self::#ident { #field_name: Box::new(value) }
				}
			} else {
				let ident = data.enum_variant_name;

				quote! {
					Self::#ident { #field_name: value }
				}
			}
		},
		GenericVariant::Tupple { is_boxed } => {
			if *is_boxed {
				let ident = data.enum_variant_name;

				quote! {
					Self::#ident(Box::new(value))
				}
			} else {
				let ident = data.enum_variant_name;

				quote! {
					Self::#ident(value)
				}
			}
		},
	}
}

fn generate_custom_not_found<'a>(data: &'a Vec<CustomError<'a>>) -> Option<&'a Ident> {
	data.iter()
		.find(|p| p.variant == CustomErrorVariant::NotFound)
		.map(|f| f.enum_variant_name)
}

fn generate_custom(data: &CustomError) -> proc_macro2::TokenStream {
	let ident = data.enum_variant_name;
	match &data.variant {
		CustomErrorVariant::NotFound => {
			quote! {}
		},
		CustomErrorVariant::Constraint(c) => {
			let literal = Literal::string(c.as_str());

			quote! {
				if let Some(_) = e.try_downcast_ref::<sqlx::postgres::PgDatabaseError>() {
					if let Some(constraint_name) = e.constraint() {
						if constraint_name == #literal {
							return Self::#ident;
						}
					}
				} else {
					if e.message().contains(#literal) {
						return Self::#ident;
					}
				}
			}
		},
		CustomErrorVariant::MessageIncludes(m) => {
			let literal = Literal::string(m.as_str());

			quote! {
				if e.message().contains(#literal) {
					return Self::#ident;
				}
			}
		},
		CustomErrorVariant::IsUnique(n) => {
			match n {
				Some(name) => {
					let literal = Literal::string(name.as_str());

					quote! {
						if e.is_foreign_key_violation() && e.message().contains(#literal) {
							return Self::#ident;
						}
					}
				},
				None => {
					quote! {
						if e.is_foreign_key_violation() {
							return Self::#ident;
						}
					}
				},
			}
		},
		CustomErrorVariant::IsForeignKey(n) => {
			match n {
				Some(name) => {
					let literal = Literal::string(name.as_str());

					quote! {
						if e.is_foreign_key_violation() && e.message().contains(#literal) {
							return Self::#ident;
						}
					}
				},
				None => {
					quote! {
						if e.is_foreign_key_violation() {
							return Self::#ident;
						}
					}
				},
			}
		},
	}
}

pub fn generate<'a>(data: &FromSqlxParsedAst<'a>) -> proc_macro2::TokenStream {
	let enum_name = data.enum_name;
	let generic_code = generate_generic(&data.generic_variant);
	let row_not_found = generate_custom_not_found(&data.custom_errors).map(|i| {
		quote! {
			sqlx::Error::RowNotFound => {
				return Self::#i;
			},
		}
	});
	let custom_code = data
		.custom_errors
		.iter()
		.filter(|c| c.variant != CustomErrorVariant::NotFound)
		.map(|c| generate_custom(c));

	quote! {
		impl From<sqlx::Error> for #enum_name {
			fn from(value: sqlx::Error) -> Self {
				match &value {
					sqlx::Error::Database(e) => {
						#(#custom_code)*

						#generic_code
					},
					#row_not_found
					_ => {
						#generic_code
					},
				}
			}
		}
	}
}
