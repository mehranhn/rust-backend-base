use std::ops::DerefMut;

use sqlx::{Connection, PgConnection, Postgres, Transaction};

use crate::external::repo::implementations::sea_query_postgres::utils::db_handle::{
	DbHandle, DbHandleInner,
};

#[derive(Debug)]
pub struct DbHandleTransaction<'a>(Transaction<'a, Postgres>);

impl<'a> DbHandleTransaction<'a> {
	pub fn new(inner: Transaction<'a, Postgres>) -> Self {
		Self(inner)
	}

	pub async fn rollback(self) -> Result<(), sqlx::Error> {
		self.0.rollback().await
	}

	pub async fn commit(self) -> Result<(), sqlx::Error> {
		self.0.commit().await
	}
}

impl<'b> DbHandleInner for DbHandleTransaction<'b> {
	fn raw_connection(&mut self) -> &mut PgConnection {
		self.0.deref_mut()
	}

	async fn tx<'a>(&'a mut self) -> Result<DbHandle<DbHandleTransaction<'a>>, sqlx::Error> {
		let tx = self.0.begin().await?;
		Ok(DbHandle::<DbHandleTransaction>::new_transaction(tx))
	}
}
