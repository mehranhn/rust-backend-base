mod api;
mod listen_address;
mod log_level;

use crate::{error::DynError, app::AppConfig};
use ron::extensions::Extensions;
use serde::Deserialize;

pub use api::ConfigApi;
pub use listen_address::{ConfigTls, ListenAddress};
pub use log_level::LogLevel;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
	#[serde(default)]
	pub log_level: LogLevel,
	pub database_url: String,
	pub api: ConfigApi,
	pub app: AppConfig,
}

impl Config {
	pub(super) async fn read_config(path: &str) -> Result<Self, DynError> {
		match tokio::fs::read(path).await {
			Ok(config_file_bytes) => match String::from_utf8(config_file_bytes) {
				Ok(str) => match ron::Options::default()
					.with_default_extension(Extensions::all())
					.from_str::<Self>(str.as_str())
				{
					Ok(r) => Ok(r),
					Err(e) => Err(Box::new(e)),
				},
				Err(e) => Err(Box::new(e)),
			},
			Err(e) => Err(Box::new(e)),
		}
	}
}
