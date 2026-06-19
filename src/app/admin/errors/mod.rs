use custom_macros::FromBoxError;
use thiserror::Error;

use crate::{
	error::DynError,
	external::repo::errors::{
		ErrExRepoAdminCreate, ErrExRepoAdminDelete, ErrExRepoAdminGetById, ErrExRepoAdminUpdate,
	},
};

#[derive(Debug, Error, FromBoxError)]
pub enum ErrSvAdminGetById {
	#[error(transparent)]
	RepoError(#[from] ErrExRepoAdminGetById),

	#[error(transparent)]
	ServerError(#[from] DynError),
}

#[derive(Debug, Error, FromBoxError)]
pub enum ErrSvAdminCreate {
	#[error(transparent)]
	RepoError(#[from] ErrExRepoAdminCreate),

	#[error(transparent)]
	ServerError(#[from] DynError),
}

#[derive(Debug, Error, FromBoxError)]
pub enum ErrSvAdminUpdate {
	#[error(transparent)]
	RepoError(#[from] ErrExRepoAdminUpdate),

	#[error(transparent)]
	ServerError(#[from] DynError),
}

#[derive(Debug, Error, FromBoxError)]
pub enum ErrSvAdminDelete {
	#[error(transparent)]
	RepoError(#[from] ErrExRepoAdminDelete),

	#[error(transparent)]
	ServerError(#[from] DynError),
}
