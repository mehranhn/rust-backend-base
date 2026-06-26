use crate::{
	app::errors::ErrServerError,
	dtos::SeedDto,
	external::{
		memory::{
			ExMemory,
			errors::{ErrExMemoryGet, ErrExMemoryUpsert},
			types::ExMTtlValue,
		},
		repo::ExRepo,
	},
};

mod admin;
mod auth;

pub mod errors {
	use std::fmt::Display;

	use crate::error::{DynError, ToBoxedError};

	#[derive(Debug)]
	pub struct ErrServerError(pub DynError);

	impl Display for ErrServerError {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			self.0.fmt(f)
		}
	}

	impl std::error::Error for ErrServerError {}

	impl From<sqlx::Error> for ErrServerError {
		fn from(value: sqlx::Error) -> Self {
			Self(Box::new(value))
		}
	}

	impl From<sea_orm::DbErr> for ErrServerError {
		fn from(value: sea_orm::DbErr) -> Self {
			Self(Box::new(value))
		}
	}

	impl From<sqlx::migrate::MigrateError> for ErrServerError {
		fn from(value: sqlx::migrate::MigrateError) -> Self {
			Self(Box::new(value))
		}
	}

	impl From<sea_query::error::Error> for ErrServerError {
		fn from(value: sea_query::error::Error) -> Self {
			Self(Box::new(value))
		}
	}

	impl ToBoxedError for ErrServerError {
		fn into_dyn_error(self) -> DynError {
			self.0
		}
	}

	pub use super::admin::errors::*;
	pub use super::auth::errors::*;
}

mod config {
	use serde::Deserialize;
	use serde_with::{serde_as, DurationSeconds, base64::Base64};

	use crate::utils::hash_password;

	#[serde_as]
	#[derive(Debug, Clone, Deserialize)]
	pub struct AppConfig {
		#[serde(default = "AppConfig::default_super_admin_username")]
		pub super_admin_username: String,

		#[serde(default = "AppConfig::default_super_admin_hashed_password")]
		#[serde_as(as = "Base64")]
		pub super_admin_hashed_password: [u8; 32],

		pub jwt_secret: String,

		#[serde(default = "AppConfig::default_jwt_exp_after")]
		#[serde_as(as = "DurationSeconds<i64>")]
		pub jwt_exp_after: time::Duration,

		#[serde(default = "AppConfig::default_session_exp_after")]
		#[serde_as(as = "DurationSeconds<i64>")]
		pub session_expire_after: time::Duration,
	}

	impl AppConfig {
		pub fn default_super_admin_username() -> String {
			String::from("admin")
		}

		pub fn default_super_admin_hashed_password() -> [u8; 32] {
			hash_password(Self::default_super_admin_username().as_str(), "admin")
				.try_into()
				.unwrap()
		}

		pub fn default_jwt_exp_after() -> time::Duration {
			time::Duration::minutes(15)
		}

		pub fn default_session_exp_after() -> time::Duration {
			time::Duration::days(180)
		}
	}
}

pub use config::AppConfig;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub struct App<D: ExRepo, M: ExMemory> {
	config: AppConfig,
	shutdown_token: CancellationToken,
	repo: D,
	memory: M,
}

impl<D: ExRepo, M: ExMemory> App<D, M> {
	pub fn new(config: AppConfig, shutdown_token: CancellationToken, repo: D, memory: M) -> Self {
		Self {
			config,
			shutdown_token,
			repo,
			memory,
		}
	}

	pub async fn init(&'static self) -> Result<(), ErrServerError> {
		self.repo.run_migrations().await?;
		self.repo
			.seed(SeedDto {
				super_admin_username: self.config.super_admin_username.clone(),
				super_admin_hashed_password: self.config.super_admin_hashed_password.to_vec(),
			})
			.await?;

		self.memory
			.run_background_workers(self.config(), self.shutdown_token.clone());

		Ok(())
	}

	pub fn shutdown_token(&self) -> CancellationToken {
		self.shutdown_token.clone()
	}

	pub fn config(&self) -> &AppConfig {
		&self.config
	}

	pub async fn session_blacklist_is_blacklist(
		&self, session_id: Uuid,
	) -> Result<bool, ErrServerError> {
		match self
			.memory
			.get(format!("session_blacklist_{session_id}").as_str())
			.await
		{
			Ok(_) => Ok(true),
			Err(e) => match e {
				ErrExMemoryGet::NotFound => Ok(false),
				ErrExMemoryGet::ServerError(e) => Err(ErrServerError(e)),
			},
		}
	}

	pub async fn session_blacklist_blacklist(
		&self, session_id: Uuid,
	) -> Result<(), ErrExMemoryUpsert> {
		self.memory
			.upsert(
				format!("session_blacklist_{session_id}").as_str(),
				String::from("1"),
				ExMTtlValue::Duration(self.config.jwt_exp_after),
			)
			.await?;

		Ok(())
	}
}
