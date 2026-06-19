use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
	utils::hash_password,
	validators::{StringVEmail, StringVPassword, StringVPhone, StringVUsername},
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AdminCreateDto<T> {
	pub username: StringVUsername,
	pub password: T,
	pub phone: Option<StringVPhone>,
	pub email: Option<StringVEmail>,
}

impl AdminCreateDto<StringVPassword> {
	pub fn into_hashed(self) -> AdminCreateDto<Vec<u8>> {
		let hashed_password = hash_password(&self.username, &self.password);
		AdminCreateDto {
			username: self.username,
			password: hashed_password,
			phone: self.phone,
			email: self.email,
		}
	}
}
