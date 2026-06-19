use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
	Off,
	Error,
	Warn,
	Info,
	Debug,
	Trace,
}

impl Default for LogLevel {
	fn default() -> Self {
		if cfg!(debug_assertions) {
			Self::Debug
		} else {
			Self::Info
		}
	}
}

impl AsRef<str> for LogLevel {
	fn as_ref(&self) -> &str {
		match self {
			LogLevel::Off => "off",
			LogLevel::Error => "error",
			LogLevel::Warn => "warn",
			LogLevel::Info => "info",
			LogLevel::Debug => "debug",
			LogLevel::Trace => "trace",
		}
	}
}

impl TryFrom<&str> for LogLevel {
	type Error = ();

	fn try_from(value: &str) -> Result<Self, <LogLevel as TryFrom<&str>>::Error> {
		match value {
			"off" => Ok(Self::Off),
			"error" => Ok(Self::Error),
			"warn" => Ok(Self::Warn),
			"info" => Ok(Self::Info),
			"debug" => Ok(Self::Debug),
			"trace" => Ok(Self::Trace),
			_ => Err(()),
		}
	}
}
