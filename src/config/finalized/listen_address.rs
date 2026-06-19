use std::net::{IpAddr, Ipv6Addr};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigTls {
	pub key: String,
	pub cert: String,
}

impl ConfigTls {
	pub fn new(key: String, cert: String) -> Self {
		Self { key, cert }
	}
}

#[derive(Debug, Clone, Deserialize)]
pub enum ListenAddress {
	#[cfg(unix)]
	Unix(String),
	Tcp {
		address: IpAddr,
		port: u16,

		#[serde(default)]
		tls: Option<ConfigTls>,

		#[cfg(target_os = "linux")]
		fwmark: Option<u16>,
	},
}

impl Default for ListenAddress {
	fn default() -> Self {
		Self::Tcp {
			address: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
			port: 3000,
			tls: None,

			#[cfg(target_os = "linux")]
			fwmark: None,
		}
	}
}
