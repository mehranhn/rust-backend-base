use custom_macros::{FromBoxError, FromSqlxError};
use thiserror::Error;

use crate::error::DynError;

#[derive(Debug, Error, FromBoxError, FromSqlxError)]
pub enum ErrExRepoAdminGetById {
	#[error("Not Found")]
	#[esqlx(not_found)]
	NotFound,

	#[error(transparent)]
	ServerError(#[from] DynError),
}

#[derive(Debug, Error, FromBoxError, FromSqlxError)]
pub enum ErrExRepoAdminCreate {
	#[error("This username already exists")]
	#[esqlx(constraint = "IDX_USERNAME")]
	UniqueUserName,

	#[error(transparent)]
	ServerError(#[from] DynError),
}

#[derive(Debug, Error, FromBoxError, FromSqlxError)]
pub enum ErrExRepoAdminUpdate {
	#[error("Not Found")]
	NotFound,

	#[error(transparent)]
	ServerError(#[from] DynError),
}

#[derive(Debug, Error, FromBoxError, FromSqlxError)]
pub enum ErrExRepoAdminDelete {
	#[error("Not Found")]
	NotFound,

	#[error(transparent)]
	ServerError(#[from] DynError),
}
