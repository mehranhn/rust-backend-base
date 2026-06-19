use custom_macros::FromBoxError;
use thiserror::Error;

use crate::error::DynError;

#[derive(Debug, Error, FromBoxError)]
pub enum ErrExMemoryGet {
	#[error("Key Not Found")]
	NotFound,

	#[error(transparent)]
	ServerError(#[from] DynError),
}

#[derive(Debug, Error, FromBoxError)]
pub enum ErrExMemorySet {
	#[error("Key Not Found")]
	KeyExists,

	#[error(transparent)]
	ServerError(#[from] DynError),
}

#[derive(Debug, Error, FromBoxError)]
pub enum ErrExMemoryFetchAddOrSet {
	#[error(transparent)]
	ServerError(#[from] DynError),
}

#[derive(Debug, Error, FromBoxError)]
pub enum ErrExMemoryUpsert {
	#[error(transparent)]
	ServerError(#[from] DynError),
}

#[derive(Debug, Error, FromBoxError)]
pub enum ErrExMemoryUpdateTtl {
	#[error("Key Not Found")]
	NotFound,

	#[error(transparent)]
	ServerError(#[from] DynError),
}

#[derive(Debug, Error, FromBoxError)]
pub enum ErrExMemoryDelete {
	#[error(transparent)]
	ServerError(#[from] DynError),
}
