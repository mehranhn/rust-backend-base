use custom_macros::{FromBoxError, FromSqlxError};
use thiserror::Error;

use crate::error::DynError;

#[derive(Debug, Error, FromBoxError, FromSqlxError)]
pub enum ErrExRepoAuthRenewSession {
	#[error("Not Found")]
	#[esqlx(not_found)]
	NotFound,

	#[error(transparent)]
	ServerError(#[from] DynError),
}
