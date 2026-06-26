use proc_macro::TokenStream;

mod generate;
mod generate_sea_orm;
mod parse;

// pub fn from_sqlx_error(input: TokenStream) -> TokenStream {
// 	let ast: syn::DeriveInput = syn::parse(input).unwrap();
//
// 	match parse::parse(&ast) {
// 		Ok(data) => generate::generate(&data).into(),
// 		Err(err) => err.into_compile_error().into(),
// 	}
// }
//
// pub fn from_sea_orm_error(input: TokenStream) -> TokenStream {
// 	let ast: syn::DeriveInput = syn::parse(input).unwrap();
//
// 	match parse::parse(&ast) {
// 		Ok(data) => generate_sea_orm::generate(&data).into(),
// 		Err(err) => err.into_compile_error().into(),
// 	}
// }

pub fn from_sql_error(input: TokenStream) -> TokenStream {
	let ast: syn::DeriveInput = syn::parse(input).unwrap();

	match parse::parse(&ast) {
		Ok(data) => {
			let sqlx = generate::generate(&data);
			let sea_orm = generate_sea_orm::generate(&data);

			let code = quote::quote! {
				#sqlx
				#sea_orm
			};

			code.into()
		},
		Err(err) => err.into_compile_error().into(),
	}
}
