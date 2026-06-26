use custom_macros::{FromBoxError, FromSqlError};
use thiserror::Error;

use crate::error::DynError;

#[derive(Debug, Error, FromBoxError, FromSqlError)]
pub enum ErrExRepoAuthRenewSession {
	#[error("Not Found")]
	#[esqlx(not_found)]
	NotFound,

	#[error(transparent)]
	ServerError(#[from] DynError),
}
