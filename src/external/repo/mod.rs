mod admin;
mod auth;
pub mod errors;
pub mod implementations;

pub use admin::ExRepoAdmin;
pub use auth::ExRepoAuth;

use crate::{dtos::SeedDto, app::errors::ErrServerError};

pub trait ExRepoTranactional: Send {
	type TxGuard<'a>: ExRepoTx<'a>
	where
		Self: 'a;

	fn transaction<'a>(
		&'a mut self,
	) -> impl Future<Output = Result<Self::TxGuard<'a>, ErrServerError>> + Send;
}

pub trait ExRepoLogic: ExRepoTranactional + ExRepoAdmin + ExRepoAuth {}

pub trait ExRepoConnection: ExRepoLogic {}

pub trait ExRepoTx<'a>: ExRepoLogic {
	fn rollback(self) -> impl Future<Output = Result<(), ErrServerError>> + Send;
	fn commit(self) -> impl Future<Output = Result<(), ErrServerError>> + Send;
}

pub trait ExRepo: Send + Sync + 'static {
	type Connection: ExRepoConnection;
	type Transaction: ExRepoTx<'static>;

	fn connection(&self) -> impl Future<Output = Result<Self::Connection, ErrServerError>> + Send;
	fn transaction(&self)
	-> impl Future<Output = Result<Self::Transaction, ErrServerError>> + Send;

	fn run_migrations(&self) -> impl Future<Output = Result<(), ErrServerError>> + Send;

	fn seed(&self, dto: SeedDto) -> impl Future<Output = Result<(), ErrServerError>> + Send;
}
