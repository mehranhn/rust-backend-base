use proc_macro2::Literal;
use syn::{DataEnum, Field, Ident, Token, Variant, parse::Parse, spanned::Spanned};

pub enum GenericVariant {
	Unit,
	Struct { field_name: Ident, is_boxed: bool },
	Tupple { is_boxed: bool },
}

pub struct Generic<'a> {
	pub enum_variant_name: &'a Ident,
	pub variant: GenericVariant,
}

#[derive(PartialEq, Eq)]
pub enum CustomErrorVariant {
	NotFound,
	Constraint(String),
	MessageIncludes(String),
	IsUnique,
	IsForeignKey,
	IsCheck,
}

impl Parse for CustomErrorVariant {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let ident = input.parse::<Ident>()?;
		match ident.to_string().as_str() {
			"not_found" => Ok(Self::NotFound),
			"constraint" => {
				let _ = input.parse::<Token![=]>()?;
				let lit = input.parse::<Literal>()?;
				let lit_str = lit.to_string();
				if lit_str.starts_with("\"") && lit_str.starts_with("\"") {
					let constraint_name = &lit_str[1..(lit_str.len() - 1)];
					Ok(Self::Constraint(String::from(constraint_name)))
				} else {
					Err(syn::Error::new(
						ident.span(),
						"token in esqlx(constraint = ...) must be a string literal",
					))
				}
			},
			"msg" => {
				let _ = input.parse::<Token![=]>()?;
				let lit = input.parse::<Literal>()?;
				let lit_str = lit.to_string();
				if lit_str.starts_with("\"") && lit_str.starts_with("\"") {
					let msg_name = &lit_str[1..(lit_str.len() - 1)];
					Ok(Self::MessageIncludes(String::from(msg_name)))
				} else {
					Err(syn::Error::new(
						ident.span(),
						"token in esqlx(msg = ...) must be a string literal",
					))
				}
			},
			"unique" => Ok(Self::IsUnique),
			"fk" => Ok(Self::IsForeignKey),
			"check" => Ok(Self::IsCheck),
			_ => Err(syn::Error::new(
				ident.span(),
				"token in esqlx(...) must be one of [not_found, constraint = \"xyz\", msg = \"xyz\", unique, fk, check]",
			)),
		}
	}
}

pub struct CustomError<'a> {
	pub enum_variant_name: &'a Ident,
	pub variant: CustomErrorVariant,
}

pub struct FromSqlxParsedAst<'a> {
	pub enum_name: &'a Ident,
	pub generic_variant: Generic<'a>,
	pub custom_errors: Vec<CustomError<'a>>,
}

fn find_generic_variant(data_enum: &DataEnum) -> Option<&Variant> {
	for variant in data_enum.variants.iter() {
		for attr in variant.attrs.iter() {
			match &attr.meta {
				syn::Meta::Path(path) => {
					if path.segments.len() == 1 && path.segments[0].ident == "dyn_error" {
						return Some(variant);
					}
				},
				syn::Meta::List(_) => {},
				syn::Meta::NameValue(_) => {},
			}
		}
	}

	for variant in data_enum.variants.iter() {
		let vn = variant.ident.to_string();
		if vn == "ServerError" || vn == "Generic" || vn == "DynError" {
			return Some(variant);
		}
	}

	None
}

fn is_boxed_error(field: &Field) -> bool {
	for attr in field.attrs.iter() {
		if let syn::Meta::Path(ref path) = attr.meta
			&& path.is_ident("no_boxed")
		{
			return false;
		};
	}

	if let syn::Type::Path(ref type_path) = field.ty
		&& type_path.path.is_ident("DynError")
	{
		return true;
	}

	if let syn::Type::Path(ref type_path) = field.ty
		&& type_path.path.segments.len() == 2
		&& type_path.path.segments[0].ident == "sqlx"
		&& type_path.path.segments[1].ident == "Error"
	{
		return false;
	}

	true
}

fn parse_generic<'a>(data_enum: &'a DataEnum) -> Result<Generic<'a>, syn::Error> {
	match find_generic_variant(data_enum) {
		Some(generic_variant) => match generic_variant.fields {
			syn::Fields::Named(ref fields_named) => {
				if fields_named.named.len() == 1 {
					Ok(Generic {
						enum_variant_name: &generic_variant.ident,
						variant: GenericVariant::Struct {
							field_name: fields_named.named[0].ident.clone().unwrap(),
							is_boxed: is_boxed_error(&fields_named.named[0]),
						},
					})
				} else {
					Err(syn::Error::new(
						fields_named.span(),
						"the enum variant must have only 1 field",
					))
				}
			},
			syn::Fields::Unnamed(ref fields_unnamed) => {
				if fields_unnamed.unnamed.len() == 1 {
					Ok(Generic {
						enum_variant_name: &generic_variant.ident,
						variant: GenericVariant::Tupple {
							is_boxed: is_boxed_error(&fields_unnamed.unnamed[0]),
						},
					})
				} else {
					Err(syn::Error::new(
						fields_unnamed.span(),
						"the enum variant must have only 1 field",
					))
				}
			},
			syn::Fields::Unit => Ok(Generic {
				enum_variant_name: &generic_variant.ident,
				variant: GenericVariant::Unit,
			}),
		},
		None => Err(syn::Error::new(
			data_enum.variants.span(),
			"specify the variant with #[dyn_error] or a variant with one of these names: ServerError, Generic, DynError",
		)),
	}
}

fn parse_custom<'a>(data_enum: &'a DataEnum) -> Result<Vec<CustomError<'a>>, syn::Error> {
	let mut result = Vec::<CustomError<'a>>::new();

	for variant in data_enum.variants.iter() {
		for attr in &variant.attrs {
			if attr.path().is_ident("esqlx") {
				match &attr.meta {
					syn::Meta::Path(_) => {},
					syn::Meta::NameValue(_) => {},
					syn::Meta::List(meta_list) => {
						let v = meta_list.parse_args::<CustomErrorVariant>()?;
						match &variant.fields {
							syn::Fields::Named(fields_named) => {
								return Err(syn::Error::new(
									fields_named.span(),
									"esqlx(...) macro can be only used on unit variants",
								));
							},
							syn::Fields::Unnamed(fields_unnamed) => {
								return Err(syn::Error::new(
									fields_unnamed.span(),
									"esqlx(...) macro can be only used on unit variants",
								));
							},
							syn::Fields::Unit => {
								result.push(CustomError {
									enum_variant_name: &variant.ident,
									variant: v,
								});
							},
						}
					},
				}
			}
		}
	}

	Ok(result)
}

pub fn parse<'a>(ast: &'a syn::DeriveInput) -> Result<FromSqlxParsedAst<'a>, syn::Error> {
	match &ast.data {
		syn::Data::Struct(_) => Err(syn::Error::new(
			ast.span(),
			"must use FromSqlxError on a enum",
		)),
		syn::Data::Enum(data_enum) => Ok(FromSqlxParsedAst {
			enum_name: &ast.ident,
			generic_variant: parse_generic(data_enum)?,
			custom_errors: parse_custom(data_enum)?,
		}),
		syn::Data::Union(_) => Err(syn::Error::new(
			ast.span(),
			"must use FromSqlxError on a enum",
		)),
	}
}
