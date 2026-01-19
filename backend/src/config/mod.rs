mod env;
mod finalized;

use crate::{error::DynError, app::AppConfig};
pub use finalized::*;

async fn read_config_ron(path: &str) -> Result<Config, DynError> {
	Config::read_config(path).await
}

async fn read_config_env() -> Result<Config, &'static str> {
	let config = Config {
		log_level: env::env_log_level().unwrap_or_default(),
		database_url: env::env_database_url().ok_or("PROVIDE DATABASE_URL")?,
		api: ConfigApi {
			host: env::env_host(),
		},
		app: AppConfig {
			super_admin_username: env::env_super_admin_username()
				.unwrap_or(AppConfig::default_super_admin_username()),
			super_admin_hashed_password: env::env_super_admin_hashed_password()
				.unwrap_or(AppConfig::default_super_admin_hashed_password()),
			jwt_secret: env::env_jwt_secret().ok_or("PROVIDE JWT_SECRET")?,
			jwt_exp_after: env::env_jwt_exp_after()
				.unwrap_or(AppConfig::default_jwt_exp_after()),
			session_expire_after: env::env_session_exp_after()
				.unwrap_or(AppConfig::default_session_exp_after()),
		},
	};

	Ok(config)
}

pub async fn read_config(path: Option<String>) -> Result<Config, String> {
	match path {
		Some(p) => read_config_ron(p.as_str()).await.map_err(|e| e.to_string()),
		None => read_config_env().await.map_err(|e| e.to_string()),
	}
}
