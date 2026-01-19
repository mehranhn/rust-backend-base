use std::{
	env,
	net::{IpAddr, Ipv6Addr},
};

use base64::{Engine, engine::general_purpose};

use crate::config::{ConfigTls, ListenAddress, LogLevel};

pub fn env_log_level() -> Option<LogLevel> {
	match env::var("LOG_LEVEL") {
		Ok(r) => LogLevel::try_from(r.as_str()).ok(),
		Err(_) => None,
	}
}

pub fn env_database_url() -> Option<String> {
	env::var("DATABASE_URL").ok()
}

#[cfg(unix)]
pub fn env_listen_unix() -> Option<String> {
	env::var("LISTEN_UNIX").ok()
}

pub fn env_listen_address() -> Option<IpAddr> {
	match env::var("LISTEN_ADDRESS") {
		Ok(a) => a.parse::<IpAddr>().ok(),
		Err(_) => None,
	}
}

pub fn env_listen_port() -> Option<u16> {
	match env::var("PORT") {
		Ok(p) => p.parse::<u16>().ok(),
		Err(_) => None,
	}
}

#[cfg(target_os = "linux")]
pub fn env_listen_fwmark() -> Option<u16> {
	match env::var("LISTEN_FWMARK") {
		Ok(fw) => fw.parse::<u16>().ok(),
		Err(_) => None,
	}
}

pub fn env_tls_key() -> Option<String> {
	env::var("TLS_KEY").ok()
}

pub fn env_tls_cert() -> Option<String> {
	env::var("TLS_CERT").ok()
}

pub fn env_tls() -> Option<ConfigTls> {
	let key = env_tls_key();
	let cert = env_tls_cert();

	if let (Some(k), Some(c)) = (key, cert) {
		Some(ConfigTls { key: k, cert: c })
	} else {
		None
	}
}

pub fn env_host() -> ListenAddress {
	#[cfg(unix)]
	if let Some(path) = env_listen_unix() {
		return ListenAddress::Unix(path);
	}

	let address = env_listen_address().unwrap_or(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)));
	let port = env_listen_port().unwrap_or(3000);

	ListenAddress::Tcp {
		address,
		port,
		tls: env_tls(),

		#[cfg(target_os = "linux")]
		fwmark: env_listen_fwmark(),
	}
}

pub fn env_super_admin_username() -> Option<String> {
	env::var("SUPER_ADMIN_USERNAME").ok()
}

pub fn env_super_admin_hashed_password() -> Option<[u8; 32]> {
	match env::var("SUPER_ADMIN_HASHED_PASSWORD") {
		Ok(v) => {
			let bytes = general_purpose::STANDARD.decode(v).ok()?;
			let arr: [u8; 32] = bytes.try_into().ok()?;
			Some(arr)
		},
		Err(_) => None,
	}
}

pub fn env_jwt_secret() -> Option<String> {
	env::var("JWT_SECRET").ok()
}

pub fn env_jwt_exp_after() -> Option<time::Duration> {
	match env::var("JWT_EXP_AFTER_SECONDS") {
		Ok(v) => match v.parse::<u64>() {
			Ok(seconds) => Some(time::Duration::seconds(seconds as i64)),
			Err(_) => None,
		},
		Err(_) => None,
	}
}

pub fn env_session_exp_after() -> Option<time::Duration> {
	match env::var("SESSION_EXP_AFTER_SECONDS") {
		Ok(v) => match v.parse::<u64>() {
			Ok(seconds) => Some(time::Duration::seconds(seconds as i64)),
			Err(_) => None,
		},
		Err(_) => None,
	}
}
