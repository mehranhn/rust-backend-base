use syn::{Field, Fields, FieldsNamed, Ident, LitStr, Meta, spanned::Spanned};

pub struct SortableParsedField<'a> {
	pub ident: &'a Ident,
	pub rename_to: Option<LitStr>,
	pub is_default: bool,
}

pub struct SortableParsedAst<'a> {
	pub struct_name: &'a Ident,
	pub variants_names: Vec<SortableParsedField<'a>>,
}

fn has_not_sortable(field: &Field) -> bool {
	for attr in field.attrs.iter() {
		if attr.path().is_ident("sortable") {
			match &attr.meta {
				Meta::Path(_) => {},
				Meta::List(meta_list) => {
					let mut is_ignore = false;
					let _ = meta_list.parse_nested_meta(|meta| {
						if meta.path.is_ident("ignore") {
							is_ignore = true;
						}
						Ok(())
					});
					return is_ignore;
				},
				Meta::NameValue(_) => {},
			}
		}
	}

	false
}

fn is_default(field: &Field) -> bool {
	for attr in field.attrs.iter() {
		if attr.path().is_ident("sortable") {
			match &attr.meta {
				Meta::Path(_) => {},
				Meta::List(meta_list) => {
					let mut is_ignore = false;
					let _ = meta_list.parse_nested_meta(|meta| {
						if meta.path.is_ident("default") {
							is_ignore = true;
						}
						Ok(())
					});
					return is_ignore;
				},
				Meta::NameValue(_) => {},
			}
		}
	}

	false
}

fn parse_rename(field: &Field) -> Option<LitStr> {
	for attr in field.attrs.iter() {
		if attr.path().is_ident("sortable") {
			match &attr.meta {
				Meta::Path(_) => {},
				Meta::List(meta_list) => {
					let mut rename: Option<LitStr> = None;
					let _ = meta_list.parse_nested_meta(|meta| {
						if meta.path.is_ident("rename") {
							let value = meta.value()?;
							let s: LitStr = value.parse()?;
							rename = Some(s);
						}
						Ok(())
					});
					return rename;
				},
				Meta::NameValue(_) => {},
			}
		}
	}

	None
}

fn parse_variants(fields_named: &'_ FieldsNamed) -> Result<Vec<SortableParsedField<'_>>, syn::Error> {
	let mut res = Vec::with_capacity(fields_named.named.len());

	for field in &fields_named.named {
		if has_not_sortable(field) {
			continue;
		}

		let is_default = is_default(field);
		let rename = parse_rename(field);

		if let Some(ref i) = field.ident {
			res.push(SortableParsedField {
				ident: i,
				rename_to: rename,
				is_default,
			});
		}
	}

	if res.is_empty() {
		return Err(syn::Error::new(
			fields_named.span(),
			"you must have atleast 1 sortable field",
		));
	}

	Ok(res)
}

pub fn parse<'a>(ast: &'a syn::DeriveInput) -> Result<SortableParsedAst<'a>, syn::Error> {
	match &ast.data {
		syn::Data::Struct(data_struct) => match data_struct.fields {
			Fields::Named(ref fields_named) => Ok(SortableParsedAst {
				struct_name: &ast.ident,
				variants_names: parse_variants(fields_named)?,
			}),
			Fields::Unnamed(_) => Err(syn::Error::new(
				ast.span(),
				"must use Sortable on a named struct",
			)),
			Fields::Unit => Err(syn::Error::new(
				ast.span(),
				"must use Sortable on a named struct",
			)),
		},
		syn::Data::Enum(_) => Err(syn::Error::new(ast.span(), "must use Sortable on a struct")),
		syn::Data::Union(_) => Err(syn::Error::new(ast.span(), "must use Sortable on a struct")),
	}
}
