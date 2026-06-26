use custom_macros::FromBoxError;
use thiserror::Error;

use crate::{
	error::DynError,
	external::{
		memory::errors::ErrExMemoryUpsert,
		repo::errors::{ErrExRepoAuthRenewSession, ErrExRepoUserGetUserByUsername},
	},
};

#[derive(Debug, Error, FromBoxError)]
pub enum ErrSvRenewSession {
	#[error(transparent)]
	RepoError(#[from] ErrExRepoAuthRenewSession),

	#[error(transparent)]
	ServerError(#[from] DynError),
}

#[derive(Debug, Error, FromBoxError)]
pub enum ErrSvAuthLogin {
	#[error("Not Found")]
	IncorrectPassword,

	#[error(transparent)]
	RepoError(#[from] ErrExRepoUserGetUserByUsername),

	#[error(transparent)]
	ServerError(#[from] DynError),
}

#[derive(Debug, Error, FromBoxError)]
pub enum ErrSvAuthLogout {
	#[error(transparent)]
	MemoError(#[from] ErrExMemoryUpsert),

	#[error(transparent)]
	ServerError(#[from] DynError),
}
