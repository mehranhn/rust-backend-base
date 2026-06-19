use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
	dtos::{AuthData, UserLoginDto},
	external::repo::errors::{ErrExRepoAuthRenewSession, ErrExRepoUserGetUserByUsername},
	app::errors::ErrServerError,
};

pub trait ExRepoAuth: Send {
	fn get_user_by_username_for_login(
		&mut self, username: &str,
	) -> impl Future<Output = Result<UserLoginDto, ErrExRepoUserGetUserByUsername>> + Send;

	fn session_get_auth(
		&mut self, session_id: Uuid,
	) -> impl Future<Output = Result<AuthData, ErrExRepoAuthRenewSession>> + Send;

	fn session_create(
		&mut self, session_id: Uuid, user_id: Uuid, expire_at: Option<OffsetDateTime>,
	) -> impl Future<Output = Result<(), ErrServerError>> + Send;

	fn session_renew(
		&mut self, session_id: Uuid,
	) -> impl Future<Output = Result<(), ErrExRepoAuthRenewSession>> + Send;

	fn session_logout(
		&mut self, session_id: Uuid,
	) -> impl Future<Output = Result<(), ErrServerError>> + Send;
}
