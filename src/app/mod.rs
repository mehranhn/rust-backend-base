use crate::{dtos::SeedDto, external::repo::ExRepo, app::errors::ErrServerError};

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
	use serde_with::serde_as;

	#[derive(Debug, Clone, Deserialize)]
	#[serde_as]
	pub struct AppConfig {
		#[serde(default = "AppConfig::default_super_admin_username")]
		pub super_admin_username: String,

		#[serde(default = "AppConfig::default_super_admin_hashed_password")]
		#[serde_as(as = "Base64")]
		pub super_admin_hashed_password: [u8; 32],

		pub jwt_secret: String,

		#[serde(default = "AppConfig::default_jwt_exp_after")]
		#[serde_as(as = "DurationSeconds<u64>")]
		pub jwt_exp_after: time::Duration,

		#[serde(default = "AppConfig::default_session_exp_after")]
		#[serde_as(as = "DurationSeconds<u64>")]
		pub session_expire_after: time::Duration,
	}

	impl AppConfig {
		pub fn default_super_admin_username() -> String {
			String::from("admin")
		}

		pub fn default_super_admin_hashed_password() -> [u8; 32] {
			// admin
			[
				0xd8, 0x24, 0x94, 0xf0, 0x5d, 0x69, 0x17, 0xba, 0x02, 0xf7, 0xaa, 0xa2, 0x96, 0x89,
				0xcc, 0xb4, 0x44, 0xbb, 0x73, 0xf2, 0x03, 0x80, 0x87, 0x6c, 0xb0, 0x5d, 0x1f, 0x37,
				0x53, 0x7b, 0x78, 0x92,
			]
		}

		pub fn default_jwt_exp_after() -> time::Duration {
			time::Duration::minutes(300)
		}

		pub fn default_session_exp_after() -> time::Duration {
			time::Duration::days(180)
		}
	}
}

pub use config::AppConfig;
use tokio_util::sync::CancellationToken;

pub struct App<D: ExRepo> {
	config: AppConfig,
	shutdown_token: CancellationToken,
	repo: D,
}

impl<D: ExRepo> App<D> {
	pub fn new(config: AppConfig, shutdown_token: CancellationToken, repo: D) -> Self {
		Self {
			config,
			repo,
			shutdown_token,
		}
	}

	pub async fn init(&self) -> Result<(), ErrServerError> {
		self.repo.run_migrations().await?;
		self.repo.seed(SeedDto {
			super_admin_username: &self.config.super_admin_username,
			super_admin_hashed_password: &self.config.super_admin_hashed_password,
		})
		.await?;

		Ok(())
	}

    pub fn shutdown_token(&self) -> CancellationToken {
        self.shutdown_token.clone()
    }

	pub fn config(&self) -> &AppConfig {
		&self.config
	}
}
