use syn::{DataEnum, Expr, Ident, LitStr, spanned::Spanned};

pub struct GeneratePermissionsParsedAst<'a> {
	pub prefix: Option<&'a LitStr>,
	pub variants_names: Vec<&'a Ident>,
}

fn parse_prefix(ast: &syn::DeriveInput) -> Option<&LitStr> {
	for attr in ast.attrs.iter() {
		if attr.path().is_ident("prefix") {
			match &attr.meta {
				syn::Meta::Path(_) => return None,
				syn::Meta::List(_) => return None,
				syn::Meta::NameValue(meta_name_value) => {
					if let Expr::Lit(e) = &meta_name_value.value {
						if let syn::Lit::Str(str) = &e.lit {
							return Some(str);
						} else {
							return None;
						}
					} else {
						return None;
					}
				},
			}
		}
	}

	None
}

fn parse_variants(data_enum: &DataEnum) -> Result<Vec<&Ident>, syn::Error> {
	let mut res = Vec::with_capacity(data_enum.variants.len());

	for variant in &data_enum.variants {
		match &variant.fields {
			syn::Fields::Named(fields_named) => {
				return Err(syn::Error::new(
					fields_named.span(),
					"the enum must have only unit variants",
				));
			},
			syn::Fields::Unnamed(fields_unnamed) => {
				return Err(syn::Error::new(
					fields_unnamed.span(),
					"the enum must have only unit variants",
				));
			},
			syn::Fields::Unit => {},
		};

		res.push(&variant.ident);
	}

	Ok(res)
}

pub fn parse<'a>(
	ast: &'a syn::DeriveInput,
) -> Result<GeneratePermissionsParsedAst<'a>, syn::Error> {
	match &ast.data {
		syn::Data::Struct(_) => Err(syn::Error::new(
			ast.span(),
			"must use GeneratePermissions on a enum",
		)),
		syn::Data::Enum(data_enum) => Ok(GeneratePermissionsParsedAst {
			prefix: parse_prefix(ast),
			variants_names: parse_variants(data_enum)?,
		}),
		syn::Data::Union(_) => Err(syn::Error::new(
			ast.span(),
			"must use GeneratePermissions on a enum",
		)),
	}
}
