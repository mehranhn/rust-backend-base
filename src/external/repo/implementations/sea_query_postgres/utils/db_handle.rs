use sqlx::{PgConnection, Postgres, Transaction, pool::PoolConnection};
use std::fmt::Debug;

use crate::{
	app::errors::ErrServerError,
	external::repo::{
		ExRepoConnection, ExRepoLogic, ExRepoTranactional, ExRepoTx,
		implementations::sea_query_postgres::utils::{
			DbHandleConnection, transaction::DbHandleTransaction,
		},
	},
};

pub trait DbHandleInner: Debug + Send {
	fn raw_connection(&mut self) -> &mut PgConnection;

	fn tx<'a>(
		&'a mut self,
	) -> impl Future<Output = Result<DbHandle<DbHandleTransaction<'a>>, sqlx::Error>> + Send;
}

#[derive(Debug)]
pub struct DbHandle<T: DbHandleInner>(T);

impl DbHandle<DbHandleConnection> {
	pub fn new_connection(c: PoolConnection<Postgres>) -> Self {
		DbHandle(DbHandleConnection::new(c))
	}
}

impl<'a> DbHandle<DbHandleTransaction<'a>> {
	pub fn new_transaction(c: Transaction<'a, Postgres>) -> Self {
		DbHandle(DbHandleTransaction::new(c))
	}
}

impl<T: DbHandleInner> DbHandle<T> {
	pub fn raw_connection(&mut self) -> &mut PgConnection {
		self.0.raw_connection()
	}

	#[allow(dead_code)]
	pub fn tx<'a>(
		&'a mut self,
	) -> impl Future<Output = Result<DbHandle<DbHandleTransaction<'a>>, sqlx::Error>> + Send {
		self.0.tx()
	}
}

impl<T: DbHandleInner> ExRepoTranactional for DbHandle<T> {
	type TxGuard<'a>
		= DbHandle<DbHandleTransaction<'a>>
	where
		T: 'a;

	async fn transaction<'a>(&'a mut self) -> Result<Self::TxGuard<'a>, ErrServerError> {
		let tx = self.0.tx().await?;
		Ok(tx)
	}
}

impl<T: DbHandleInner> ExRepoLogic for DbHandle<T> {}

impl ExRepoConnection for DbHandle<DbHandleConnection> {}

impl<'a> ExRepoTx<'a> for DbHandle<DbHandleTransaction<'a>> {
	async fn rollback(self) -> Result<(), ErrServerError> {
		self.0.rollback().await?;
		Ok(())
	}

	async fn commit(self) -> Result<(), ErrServerError> {
		self.0.commit().await?;
		Ok(())
	}
}
