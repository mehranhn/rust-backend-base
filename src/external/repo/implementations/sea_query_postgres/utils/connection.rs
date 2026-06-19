use std::ops::DerefMut;

use sqlx::{Acquire, PgConnection, Postgres, pool::PoolConnection};

use crate::{
	external::repo::{
		ExRepoConnection, ExRepoLogic, ExRepoTranactional,
		implementations::sea_query_postgres::utils::ExRepoImplSeaQueryPgTx,
	},
	app::errors::ErrServerError,
};

use super::ExRepoImplSeaQueryHandle;

pub struct ExRepoImplSeaQueryPgConnection {
	connection: PoolConnection<Postgres>,
}

impl ExRepoImplSeaQueryPgConnection {
	pub fn new(connection: PoolConnection<Postgres>) -> Self {
		Self { connection }
	}
}

impl ExRepoTranactional for ExRepoImplSeaQueryPgConnection {
	type TxGuard<'a> = ExRepoImplSeaQueryPgTx<'a>;

	async fn transaction<'a>(&'a mut self) -> Result<Self::TxGuard<'a>, ErrServerError> {
		let tx = self.connection.begin().await?;
		Ok(ExRepoImplSeaQueryPgTx::new(tx))
	}
}

impl ExRepoLogic for ExRepoImplSeaQueryPgConnection {}

impl ExRepoConnection for ExRepoImplSeaQueryPgConnection {}

impl ExRepoImplSeaQueryHandle for ExRepoImplSeaQueryPgConnection {
	fn raw_connection(&mut self) -> &mut PgConnection {
		self.connection.deref_mut()
	}

	async fn tx<'a>(&'a mut self) -> Result<ExRepoImplSeaQueryPgTx<'a>, ErrServerError> {
		let tx = self.connection.begin().await?;
		Ok(ExRepoImplSeaQueryPgTx::new(tx))
	}
}
