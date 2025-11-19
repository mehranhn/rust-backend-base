use syn::{DataEnum, Ident, Variant, spanned::Spanned};

pub enum FromDynErrorParsedAstVariant<'a> {
	Named(&'a Ident),
	Unnamed,
	Unit,
}

pub struct FromDynErrorParsedAst<'a> {
	pub enum_ident: &'a Ident,
	pub variant_ident: &'a Ident,
	pub variant: FromDynErrorParsedAstVariant<'a>,
}

impl<'a> FromDynErrorParsedAst<'a> {
	fn new(enum_ident: &'a Ident, variant_ident: &'a Ident, variant: FromDynErrorParsedAstVariant<'a>) -> Self {
		Self {
			enum_ident,
			variant_ident,
			variant,
		}
	}
}

fn find_target_variant(data_enum: &DataEnum) -> Option<&Variant> {
	for variant in data_enum.variants.iter() {
		for attr in variant.attrs.iter() {
			match &attr.meta {
				syn::Meta::Path(path) => {
					if path.segments.len() == 1 && path.segments[0].ident.to_string() == "dyn_error"
					{
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

pub fn parse<'a>(ast: &'a syn::DeriveInput) -> Result<FromDynErrorParsedAst<'a>, syn::Error> {
	match &ast.data {
		syn::Data::Struct(data_struct) => Err(syn::Error::new(
			data_struct.struct_token.span(),
			"must use FromBoxError on a enum",
		)),
		syn::Data::Enum(data_enum) => match find_target_variant(&data_enum) {
			Some(variant) => match &variant.fields {
				syn::Fields::Named(fields_named) => {
					if fields_named.named.len() == 1 {
						match &fields_named.named[0].ident {
							Some(ident) => {
                                Ok(FromDynErrorParsedAst::new(
                                    &ast.ident,
                                    &variant.ident,
                                    FromDynErrorParsedAstVariant::Named(ident),
                                ))
                            },
							None => Err(syn::Error::new(
								fields_named.span(),
								"the enum variant must have only 1 field",
							)),
						}
					} else {
						Err(syn::Error::new(
							fields_named.span(),
							"the enum variant must have only 1 field",
						))
					}
				},
				syn::Fields::Unnamed(fields_unnamed) => {
					if fields_unnamed.unnamed.len() == 1 {
						Ok(FromDynErrorParsedAst::new(
							&ast.ident,
							&variant.ident,
							FromDynErrorParsedAstVariant::Unnamed,
						))
					} else {
						Err(syn::Error::new(
							fields_unnamed.span(),
							"the enum variant must have only 1 field",
						))
					}
				},
				syn::Fields::Unit => Ok(FromDynErrorParsedAst::new(
					&ast.ident,
					&variant.ident,
					FromDynErrorParsedAstVariant::Unit,
				)),
			},
			None => Err(syn::Error::new(
				data_enum.variants.span(),
				"specify the variant with #[dyn_error] or a variant with one of these names: ServerError, Generic, DynError",
			)),
		},
		syn::Data::Union(data_union) => Err(syn::Error::new(
			data_union.union_token.span(),
			"must use FromBoxError on a enum",
		)),
	}
}
