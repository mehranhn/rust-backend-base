use sea_query::enum_def;
use sqlx::prelude::FromRow;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::dtos::{AdminDto, UserLoginDto};

use super::super::types::Roles;

#[enum_def(table_name = "users")]
#[derive(Debug, FromRow)]
pub struct User {
	id: Uuid,
	created_at: PrimitiveDateTime,
	role: Roles,
	username: String,
	hashed_password: Vec<u8>,
	email: Option<String>,
	phone: Option<String>,
}

impl Into<UserLoginDto> for User {
	fn into(self) -> UserLoginDto {
		UserLoginDto {
			id: self.id,
			username: self.username,
			hashed_password: self.hashed_password,
			role: self.role.into(),
		}
	}
}

impl Into<AdminDto> for User {
	fn into(self) -> AdminDto {
		AdminDto {
			id: self.id,
			created_at: self.created_at.assume_utc(),
			username: self.username,
			phone: self.phone,
			email: self.email,
		}
	}
}
