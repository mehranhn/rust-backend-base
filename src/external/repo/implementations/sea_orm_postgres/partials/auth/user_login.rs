use sea_orm::DerivePartialModel;

use crate::{
	dtos::UserLoginDto, external::repo::implementations::sea_orm_postgres::types::roles::Roles,
};

#[derive(DerivePartialModel)]
#[sea_orm(entity = "super::super::super::models::user::Entity")]
pub struct PartialUserLogin {
	pub id: uuid::Uuid,
	pub role: Roles,
	pub username: String,
	pub hashed_password: Vec<u8>,
}

impl Into<UserLoginDto> for PartialUserLogin {
	fn into(self) -> UserLoginDto {
		UserLoginDto {
			id: self.id,
			username: self.username,
			hashed_password: self.hashed_password,
			role: self.role.into(),
		}
	}
}
