use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
	app::App,
	dtos::{AuthData, LoginDto},
	external::{
		memory::ExMemory,
		repo::{ExRepo, ExRepoAuth, errors::ErrExRepoAuthRenewSession},
	},
	utils::{check_password, generate_uuid},
};

pub mod errors;
mod utils;

impl<D: ExRepo, M: ExMemory> App<D, M> {
	pub async fn renew_session(
		&self, session_id: Uuid,
	) -> Result<AuthData, errors::ErrSvRenewSession> {
		let mut c = self.repo.connection().await?;
		let auth_data = c.session_get_auth(session_id).await?;

		if let Some(ref exp) = auth_data.expire_at {
			if *exp > OffsetDateTime::now_utc() {
				c.session_renew(session_id).await?;
				Ok(auth_data)
			} else {
				Err(errors::ErrSvRenewSession::RepoError(
					ErrExRepoAuthRenewSession::NotFound,
				))
			}
		} else {
			c.session_renew(session_id).await?;
			Ok(auth_data)
		}
	}

	pub async fn login(&self, dto: LoginDto) -> Result<AuthData, errors::ErrSvAuthLogin> {
		let mut c = self.repo.connection().await?;
		let user = c.get_user_by_username_for_login(&dto.username).await?;

		if !check_password(
			dto.username.as_str(),
			dto.password.as_str(),
			user.hashed_password.as_slice(),
		) {
			return Err(errors::ErrSvAuthLogin::IncorrectPassword);
		}

		let session_id = generate_uuid();
		let expire_at = Some(OffsetDateTime::now_utc() + self.config.session_expire_after);

		c.session_create(session_id, user.id, expire_at).await?;

		Ok(AuthData {
			user_id: user.id,
			session_id,
			role: user.role,
			username: user.username,
			expire_at,
		})
	}

	pub async fn logout(&self, session_id: Uuid) -> Result<(), errors::ErrSvAuthLogout> {
		let mut c = self.repo.connection().await?;
		c.session_logout(session_id).await?;
		self.session_blacklist_blacklist(session_id).await?;

		Ok(())
	}
}
