use std::{error::Error as StdError, num::ParseIntError, string::FromUtf8Error};

use redis::RedisError;
use tokio::task::JoinError;

use crate::{error::DynError};

pub trait ToBoxedError: StdError + Send + 'static {
	fn into_dyn_error(self) -> DynError;
}

impl ToBoxedError for tokio::io::Error {
	fn into_dyn_error(self) -> DynError {
		Box::new(self)
	}
}

impl ToBoxedError for FromUtf8Error {
	fn into_dyn_error(self) -> DynError {
		Box::new(self)
	}
}

impl ToBoxedError for JoinError {
	fn into_dyn_error(self) -> DynError {
		Box::new(self)
	}
}

impl ToBoxedError for RedisError {
	fn into_dyn_error(self) -> DynError {
		Box::new(self)
	}
}

impl ToBoxedError for ParseIntError {
	fn into_dyn_error(self) -> DynError {
		Box::new(self)
	}
}

impl ToBoxedError for sea_query::error::Error {
	fn into_dyn_error(self) -> DynError {
		Box::new(self)
	}
}
