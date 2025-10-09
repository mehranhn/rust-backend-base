use proc_macro::TokenStream;
use syn::{Item, parse_macro_input};

mod parse;
mod state;
mod utils;

#[proc_macro_attribute]
pub fn variants(attrs: TokenStream, item: TokenStream) -> TokenStream {
	let item = parse_macro_input!(item as Item);
	// println!("{attrs:#?}");
	match item {
		Item::Enum(item_enum) => syn::Error::new_spanned(
			item_enum.enum_token,
			"#[variants] not implemented for enums yet",
		)
		.into_compile_error()
		.into(),
		Item::Struct(item_struct) => {
			println!("{item_struct:#?}");

			quote::quote! {
				#item_struct
			}
			.into()
		},
		other => {
			syn::Error::new_spanned(other, "#[variants] can only be applied to structs or enums")
				.into_compile_error()
				.into()
		},
	}
	//
	// quote::quote! {}.into()
	// match parse_state(input) {
	// 	Ok(s) => {
	// 		// println!("hooooooooooy {s:#?}");
	// 		quote::quote! {}.into()
	// 	},
	// 	Err(e) => e.into_compile_error().into(),
	// }
}
