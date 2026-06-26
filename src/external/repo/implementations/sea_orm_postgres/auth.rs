use std::marker::Send;

use sea_orm::{ActiveValue, EntityTrait, QueryFilter, QuerySelect};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
	app::errors::ErrServerError,
	dtos::{AuthData, UserLoginDto},
	external::repo::{
		auth::ExRepoAuth,
		errors::{ErrExRepoAuthRenewSession, ErrExRepoUserGetUserByUsername},
		implementations::sea_orm_postgres::{
			models,
			partials::{PartialAuthData, PartialUserLogin},
			utils::{DbHandle, DbHandleInner},
		},
	},
};

impl<T: DbHandleInner + Send> ExRepoAuth for DbHandle<T> {
	async fn get_user_by_username_for_login(
		&mut self, username: &str,
	) -> Result<UserLoginDto, ErrExRepoUserGetUserByUsername> {
		let result = models::user::Entity::find_by_username(username)
			.into_partial_model::<PartialUserLogin>()
			.one(self)
			.await?
			.ok_or(ErrExRepoUserGetUserByUsername::NotFound)?;

		Ok(result.into())
	}

	async fn session_get_auth(
		&mut self, session_id: Uuid,
	) -> Result<AuthData, ErrExRepoAuthRenewSession> {
		let Some(result) = models::session::Entity::find_by_id(session_id)
			.select_only()
			.left_join(models::user::Entity)
			.filter(models::session::COLUMN.deleted_at.is_null())
			.into_partial_model::<PartialAuthData>()
			.one(self)
			.await?
		else {
			return Err(ErrExRepoAuthRenewSession::NotFound);
		};

		Ok(result.into())
	}

	async fn session_create(
		&mut self, session_id: Uuid, user_id: Uuid, expire_at: Option<OffsetDateTime>,
	) -> Result<(), ErrServerError> {
		let active_model = models::session::ActiveModel {
			id: ActiveValue::Set(session_id),
			created_at: ActiveValue::Set(OffsetDateTime::now_utc()),
			updated_at: ActiveValue::Set(OffsetDateTime::now_utc()),
			user_id: ActiveValue::Set(user_id),
			expire_at: ActiveValue::Set(expire_at),
			last_access: ActiveValue::Set(OffsetDateTime::now_utc()),
			..Default::default()
		};

		models::session::Entity::insert(active_model)
			.exec(self)
			.await?;

		Ok(())
	}

	async fn session_renew(&mut self, session_id: Uuid) -> Result<(), ErrExRepoAuthRenewSession> {
		let active_model = models::session::ActiveModel {
			last_access: ActiveValue::Set(OffsetDateTime::now_utc()),
			..Default::default()
		};

		models::session::Entity::update_many()
			.set(active_model)
			.filter(models::session::COLUMN.id.eq(session_id))
			.filter(models::session::COLUMN.deleted_at.is_null())
			.exec(self)
			.await?;

		Ok(())
	}

	async fn session_logout(&mut self, session_id: Uuid) -> Result<(), ErrServerError> {
		let active_model = models::session::ActiveModel {
			deleted_at: ActiveValue::Set(Some(OffsetDateTime::now_utc())),
			..Default::default()
		};

		models::session::Entity::update_many()
			.set(active_model)
			.filter(models::session::COLUMN.id.eq(session_id))
			.filter(models::session::COLUMN.deleted_at.is_null())
			.exec(self)
			.await?;

		Ok(())
	}
}
