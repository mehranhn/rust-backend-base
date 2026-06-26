use sea_orm::DerivePartialModel;
use time::OffsetDateTime;

use crate::{
	dtos::AuthData, external::repo::implementations::sea_orm_postgres::types::roles::Roles,
};

#[derive(DerivePartialModel)]
#[sea_orm(entity = "super::super::models::session::Entity")]
pub struct PartialAuthData {
	pub id: uuid::Uuid,
	pub expire_at: Option<OffsetDateTime>,

	#[sea_orm(nested)]
	pub user: PartialAuthDataUser,
}

#[derive(DerivePartialModel)]
#[sea_orm(entity = "super::super::models::user::Entity")]
pub struct PartialAuthDataUser {
	pub id: uuid::Uuid,
	pub role: Roles,
	pub username: String,
}

impl Into<AuthData> for PartialAuthData {
	fn into(self) -> AuthData {
		AuthData {
			user_id: self.user.id,
			session_id: self.id,
			role: self.user.role.into(),
			username: self.user.username,
			expire_at: self.expire_at,
		}
	}
}
