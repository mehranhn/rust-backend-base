use crate::modules::user::externals::repo::ExRepoUserModule;

mod errors;
mod implementations;

pub trait ExRepoTx {
	async fn rollback(self);
	async fn commit(self);
}

pub trait ExRepoBase: Send + Sync + 'static {
	type Connection;
	type Tx: ExRepoTx + AsMut<Self::Connection>;

	async fn connection(&self) -> Self::Connection;
	async fn transaction(&self) -> Self::Tx;
}

pub trait ExRepo: ExRepoBase + ExRepoUserModule {}
