mod variants;

use std::collections::HashMap;

use syn::{Attribute, Field, Generics, Ident, Visibility};

use variants::StateVariant;

#[derive(Debug)]
pub struct State {
	pub attrs: Vec<Attribute>,
	pub vis: Visibility,
	pub generics: Generics,
	pub fields: HashMap<Ident, Field>,
	pub variants: Vec<StateVariant>,
}

impl State {
	pub fn new(
		attrs: Vec<Attribute>, vis: Visibility, generics: Generics, fields: HashMap<Ident, Field>,
		variants: Vec<StateVariant>,
	) -> Self {
		Self {
			attrs,
			vis,
			generics,
			fields,
			variants,
		}
	}
}
