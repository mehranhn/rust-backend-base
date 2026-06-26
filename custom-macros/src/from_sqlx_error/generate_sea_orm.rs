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

fn generate_custom_constraint_or_message(data: CustomErrorConstraintOrMsg) -> proc_macro2::TokenStream {
	let ident = data.enum_variant_name;
	let literal = Literal::string(data.error_name.as_str());

	quote! {
		if value.to_string().contains(#literal) {
			return Self::#ident;
		}
	}
}

pub struct CustomErrorUniqueOrFk<'a> {
	pub enum_variant_name: &'a Ident,
	pub error_name: Option<String>,
}

pub struct CustomErrorConstraintOrMsg<'a> {
	pub enum_variant_name: &'a Ident,
	pub error_name: String,
}

fn generate_unique(unique_errors: Vec<CustomErrorUniqueOrFk<'_>>) -> proc_macro2::TokenStream {
	if !unique_errors.is_empty() {
		let first_no_msg = unique_errors
			.iter()
			.find(|e| e.error_name.is_none())
			.map(|e| {
				let a = e.enum_variant_name;
				quote! { return Self::#a; }
			});
		let count_of_named = unique_errors
			.iter()
			.filter(|e| e.error_name.is_some())
			.count();
		let named = unique_errors.iter().filter_map(|e| match &e.error_name {
			Some(en) => {
				let literal = Literal::string(en.as_str());
				let a = e.enum_variant_name;

				Some(quote! {
					if e == #literal {
						return Self::#a;
					}
				})
			},
			None => None,
		});

		let if_tokens = if count_of_named > 0 {
			quote! { sea_orm::SqlErr::UniqueConstraintViolation(e) }
		} else {
			quote! { sea_orm::SqlErr::UniqueConstraintViolation(_) }
		};

		quote! {
			#if_tokens => {
				#(#named)*
				#first_no_msg
			}
		}
	} else {
		quote! {}
	}
}

fn generate_fk(fk_errors: Vec<CustomErrorUniqueOrFk<'_>>) -> proc_macro2::TokenStream {
	if !fk_errors.is_empty() {
		let first_no_msg = fk_errors
			.iter()
			.find(|e| e.error_name.is_none())
			.map(|e| {
				let a = e.enum_variant_name;
				quote! { return Self::#a; }
			});
		let count_of_named = fk_errors
			.iter()
			.filter(|e| e.error_name.is_some())
			.count();
		let named = fk_errors.iter().filter_map(|e| match &e.error_name {
			Some(en) => {
				let literal = Literal::string(en.as_str());
				let a = e.enum_variant_name;

				Some(quote! {
					if e == #literal {
						return Self::#a;
					}
				})
			},
			None => None,
		});

		let if_tokens = if count_of_named > 0 {
			quote! { sea_orm::SqlErr::ForeignKeyConstraintViolation(e) }
		} else {
			quote! { sea_orm::SqlErr::ForeignKeyConstraintViolation(_) }
		};

		quote! {
			#if_tokens => {
				#(#named)*
				#first_no_msg
			}
		}
	} else {
		quote! {}
	}
}

fn generate_is_sql_err_check(
	unique_errors: Vec<CustomErrorUniqueOrFk<'_>>, fk_errors: Vec<CustomErrorUniqueOrFk<'_>>,
) -> proc_macro2::TokenStream {
	if unique_errors.len() + fk_errors.len() > 0 {
		let if_one_empty = if unique_errors.is_empty() || fk_errors.is_empty() {
			quote! {
				_ => {},
			}
		} else {
			quote! {}
		};

		let unique = generate_unique(unique_errors);
		let fk = generate_fk(fk_errors);

		quote! {
			match value.sql_err() {
				sea_orm::SqlErr::UniqueConstraintViolation(e) => {
				},
				sea_orm::SqlErr::ForeignKeyConstraintViolation(e) => {
				},
				#unique
				#fk
				#if_one_empty
			}
		}
	} else {
		quote! {}
	}
}

pub fn generate<'a>(data: &FromSqlxParsedAst<'a>) -> proc_macro2::TokenStream {
	let enum_name = data.enum_name;
	let generic_code = generate_generic(&data.generic_variant);
	let row_not_found = generate_custom_not_found(&data.custom_errors).map(|i| {
		quote! {
			if let sea_orm::DbErr::RecordNotUpdated | sea_orm::DbErr::RecordNotFound(_) = value {
				return Self::#i;
			}
		}
	});

	let unique_errors: Vec<_> = data
		.custom_errors
		.iter()
		.filter_map(|c| match &c.variant {
			CustomErrorVariant::IsUnique(e) => Some(CustomErrorUniqueOrFk {
				enum_variant_name: c.enum_variant_name,
				error_name: e.clone(),
			}),
			_ => None,
		})
		.collect();

	let fk_errors: Vec<_> = data
		.custom_errors
		.iter()
		.filter_map(|c| match &c.variant {
			CustomErrorVariant::IsUnique(e) => Some(CustomErrorUniqueOrFk {
				enum_variant_name: c.enum_variant_name,
				error_name: e.clone(),
			}),
			_ => None,
		})
		.collect();

	let sql_err = generate_is_sql_err_check(unique_errors, fk_errors);

	let custom_code = data
		.custom_errors
		.iter()
		.filter_map(|c| match &c.variant {
			CustomErrorVariant::Constraint(e) => Some(CustomErrorConstraintOrMsg{
				enum_variant_name: c.enum_variant_name,
				error_name: e.clone(),
			}),
			CustomErrorVariant::MessageIncludes(e) => Some(CustomErrorConstraintOrMsg{
				enum_variant_name: c.enum_variant_name,
				error_name: e.clone(),
			}),
			_ => None,
		})
		.map(|c| generate_custom_constraint_or_message(c));

	quote! {
		impl From<sea_orm::DbErr> for #enum_name {
			fn from(value: sea_orm::DbErr) -> Self {
				#sql_err
				#row_not_found
				#(#custom_code)*
				#generic_code
			}
		}
	}
}
