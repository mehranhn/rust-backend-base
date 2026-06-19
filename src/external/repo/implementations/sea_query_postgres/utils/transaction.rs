use std::ops::DerefMut;

use sqlx::{Acquire, PgConnection, Postgres, Transaction};

use crate::{
	external::repo::{
		ExRepoLogic, ExRepoTranactional, ExRepoTx,
		implementations::sea_query_postgres::utils::ExRepoImplSeaQueryHandle,
	},
	app::errors::ErrServerError,
};

pub struct ExRepoImplSeaQueryPgTx<'a> {
	inner: Transaction<'a, Postgres>,
}

impl<'a> ExRepoImplSeaQueryPgTx<'a> {
	pub fn new(inner: Transaction<'a, Postgres>) -> Self {
		Self { inner }
	}
}

impl ExRepoTranactional for ExRepoImplSeaQueryPgTx<'_> {
	type TxGuard<'b>
		= ExRepoImplSeaQueryPgTx<'b>
	where
		Self: 'b;

	async fn transaction<'b>(&'b mut self) -> Result<Self::TxGuard<'b>, ErrServerError> {
		let tx = self.inner.begin().await?;
		Ok(ExRepoImplSeaQueryPgTx::new(tx))
	}
}

impl ExRepoLogic for ExRepoImplSeaQueryPgTx<'_> {}

impl<'a> ExRepoTx<'a> for ExRepoImplSeaQueryPgTx<'a> {
	async fn rollback(self) -> Result<(), ErrServerError> {
		self.inner.rollback().await?;
		Ok(())
	}

	async fn commit(self) -> Result<(), ErrServerError> {
		self.inner.commit().await?;
		Ok(())
	}
}

impl ExRepoImplSeaQueryHandle for ExRepoImplSeaQueryPgTx<'_> {
	fn raw_connection(&mut self) -> &mut PgConnection {
		self.inner.deref_mut()
	}

	async fn tx<'a>(&'a mut self) -> Result<ExRepoImplSeaQueryPgTx<'a>, ErrServerError> {
		let tx = self.inner.begin().await?;
		Ok(ExRepoImplSeaQueryPgTx::new(tx))
	}
}
