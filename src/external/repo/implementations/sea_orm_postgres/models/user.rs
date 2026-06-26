use sea_orm::entity::prelude::*;
use time::OffsetDateTime;

use crate::{dtos::AdminDto, external::repo::implementations::sea_orm_postgres::types::roles::Roles};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: uuid::Uuid,

	#[sea_orm(default_value = "CURRENT_TIMESTAMP")]
	pub created_at: OffsetDateTime,

	#[sea_orm(default_value = "CURRENT_TIMESTAMP")]
	pub updated_at: OffsetDateTime,

	#[sea_orm(unique, column_type = "Integer")]
	pub role: Roles,

	#[sea_orm(unique, column_type = "String(StringLen::N(255))")]
	pub username: String,

	#[sea_orm(default_value = "Blob")]
	pub hashed_password: Vec<u8>,

	#[sea_orm(column_type = "String(StringLen::N(21))")]
	pub phone: Option<String>,

	#[sea_orm(column_type = "String(StringLen::N(255))")]
	pub email: Option<String>,

	#[sea_orm(has_many)]
	pub sessions: HasMany<super::session::Entity>,

	#[sea_orm(has_many)]
	pub posts: HasMany<super::post::Entity>,

	#[sea_orm(has_many)]
	pub comments: HasMany<super::comment::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

impl Into<AdminDto> for Model {
	fn into(self) -> AdminDto {
		AdminDto {
			id: self.id,
			created_at: self.created_at,
			username: self.username,
			phone: self.phone,
			email: self.email,
		}
	}
}
