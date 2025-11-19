use proc_macro2::Literal;
use syn::{Ident, Token, Type, Variant, parse::Parse, spanned::Spanned};

#[derive(Debug)]
pub enum AxumResponseStatus {
	Integer(u16),
	Ident(Ident),
}

impl<'a> Parse for AxumResponseStatus {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let status_ident = input.parse::<Ident>()?;
		if status_ident == "status" {
			let _ = input.parse::<Token![=]>()?;
			let result = match input.parse::<Ident>() {
				Ok(ident) => Ok(Self::Ident(ident.clone())),
				Err(_) => {
					let literal = input.parse::<Literal>()?;
					let lit_str = literal.to_string();
					match lit_str.parse::<u16>() {
						Ok(integer) => {
							if integer < 100 || integer > 999 {
								return Err(syn::Error::new(
									input.span(),
									"status must be between 100 and 999",
								));
							}
							Ok(Self::Integer(integer))
						},
						Err(_) => {
							Err(syn::Error::new(
								input.span(),
								"status should not be string",
							))
						},
					}
				},
			};

			let _ = input.step(|cursor| {
				let mut rest = *cursor;
				while let Some((_, next)) = rest.token_tree() {
					rest = next;
				}

				Ok(((), rest))
			});

			result
		} else {
			Err(syn::Error::new(
				input.span(),
				"it must start with status = ...",
			))
		}
	}
}

#[derive(Debug)]
pub struct AxumResponseVariant<'a> {
	pub variant_name: &'a Ident,
	pub status: AxumResponseStatus,
	pub body: Option<&'a Type>,
}

#[derive(Debug)]
pub struct AxumResponseParsedAst<'a> {
	pub enum_name: &'a Ident,
	pub variants: Vec<AxumResponseVariant<'a>>,
}

fn parse_status(variant: &Variant) -> Result<AxumResponseStatus, syn::Error> {
	for attr in &variant.attrs {
		match &attr.meta {
			syn::Meta::List(meta_list) => {
				if meta_list.path.is_ident("response") {
					return meta_list.parse_args::<AxumResponseStatus>();
				}
			},
			_ => {},
		}
	}

	Err(syn::Error::new(
		variant.span(),
		"must provide a #[response(status = ...)] on every variant",
	))
}

fn parse_variant<'a>(variant: &'a Variant) -> Result<AxumResponseVariant<'a>, syn::Error> {
	let status = parse_status(variant)?;
	let body = match &variant.fields {
		syn::Fields::Named(fields_named) => {
			return Err(syn::Error::new(
				fields_named.span(),
				"it should be a unit on unnamed enum variant",
			));
		},
		syn::Fields::Unnamed(fields_unnamed) => {
			if fields_unnamed.unnamed.len() == 1 {
				Some(&fields_unnamed.unnamed[0].ty)
			} else {
				return Err(syn::Error::new(
					fields_unnamed.span(),
					"it should have only 1 unnamed field",
				));
			}
		},
		syn::Fields::Unit => None,
	};

	Ok(AxumResponseVariant {
		variant_name: &variant.ident,
		status,
		body,
	})
}

pub fn parse<'a>(ast: &'a syn::DeriveInput) -> Result<AxumResponseParsedAst<'a>, syn::Error> {
	match &ast.data {
		syn::Data::Struct(data_struct) => Err(syn::Error::new(
			data_struct.struct_token.span(),
			"must use AxumResponse on a enum",
		)),
		syn::Data::Enum(data_enum) => {
			let mut variants = Vec::with_capacity(data_enum.variants.len());

			for variant in data_enum.variants.iter() {
				let parsed_variant = parse_variant(variant)?;
				variants.push(parsed_variant);
			}

			Ok(AxumResponseParsedAst {
				enum_name: &ast.ident,
				variants,
			})
		},
		syn::Data::Union(data_union) => Err(syn::Error::new(
			data_union.union_token.span(),
			"must use AxumResponse on a enum",
		)),
	}
}
