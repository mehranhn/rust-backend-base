use serde::Deserialize;

use crate::config::finalized::listen_address::ListenAddress;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigApi {
	#[serde(default)]
	pub host: ListenAddress,
}
