use std::marker::PhantomData;

use sea_orm::{
	ConnectionTrait, DatabaseTransaction, DbBackend, DbErr, ExecResult, QueryResult, Statement,
	StatementBuilder, TransactionTrait,
};

use super::db_handle::{DbHandle, DbHandleInner};

pub struct DbHandleTransaction<'a>(DatabaseTransaction, PhantomData<&'a ()>);

impl<'a> DbHandleTransaction<'a> {
	pub fn new(tx: DatabaseTransaction) -> Self {
		Self(tx, PhantomData)
	}

	pub async fn rollback(self) -> Result<(), DbErr> {
		self.0.rollback().await
	}

	pub async fn commit(self) -> Result<(), DbErr> {
		self.0.commit().await
	}
}

impl<'b> DbHandleInner for DbHandleTransaction<'b> {
	async fn tx<'a>(&'a mut self) -> Result<DbHandle<DbHandleTransaction<'a>>, sea_orm::DbErr> {
		let tx = self.0.begin().await?;
		Ok(DbHandle::<DbHandleTransaction>::new_transaction(tx))
	}
}

impl<'a> ConnectionTrait for DbHandleTransaction<'a> {
	fn get_database_backend(&self) -> DbBackend {
		self.0.get_database_backend()
	}

	fn execute_raw<'life0, 'async_trait>(
		&'life0 self, stmt: Statement,
	) -> ::core::pin::Pin<
		Box<
			dyn ::core::future::Future<Output = Result<ExecResult, DbErr>>
				+ ::core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		self.0.execute_raw(stmt)
	}

	fn execute_unprepared<'life0, 'life1, 'async_trait>(
		&'life0 self, sql: &'life1 str,
	) -> ::core::pin::Pin<
		Box<
			dyn ::core::future::Future<Output = Result<ExecResult, DbErr>>
				+ ::core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		'life1: 'async_trait,
		Self: 'async_trait,
	{
		self.0.execute_unprepared(sql)
	}

	fn query_one_raw<'life0, 'async_trait>(
		&'life0 self, stmt: Statement,
	) -> ::core::pin::Pin<
		Box<
			dyn ::core::future::Future<Output = Result<Option<QueryResult>, DbErr>>
				+ ::core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		self.0.query_one_raw(stmt)
	}

	fn query_all_raw<'life0, 'async_trait>(
		&'life0 self, stmt: Statement,
	) -> ::core::pin::Pin<
		Box<
			dyn ::core::future::Future<Output = Result<Vec<QueryResult>, DbErr>>
				+ ::core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		'life0: 'async_trait,
		Self: 'async_trait,
	{
		self.0.query_all_raw(stmt)
	}

	fn execute<'life0, 'life1, 'async_trait, S>(
		&'life0 self, stmt: &'life1 S,
	) -> ::core::pin::Pin<
		Box<
			dyn ::core::future::Future<Output = Result<ExecResult, DbErr>>
				+ ::core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		S: 'async_trait + StatementBuilder,
		'life0: 'async_trait,
		'life1: 'async_trait,
		Self: 'async_trait,
	{
		self.0.execute(stmt)
	}

	fn query_one<'life0, 'life1, 'async_trait, S>(
		&'life0 self, stmt: &'life1 S,
	) -> ::core::pin::Pin<
		Box<
			dyn ::core::future::Future<Output = Result<Option<QueryResult>, DbErr>>
				+ ::core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		S: 'async_trait + StatementBuilder,
		'life0: 'async_trait,
		'life1: 'async_trait,
		Self: 'async_trait,
	{
		self.0.query_one(stmt)
	}

	fn query_all<'life0, 'life1, 'async_trait, S>(
		&'life0 self, stmt: &'life1 S,
	) -> ::core::pin::Pin<
		Box<
			dyn ::core::future::Future<Output = Result<Vec<QueryResult>, DbErr>>
				+ ::core::marker::Send
				+ 'async_trait,
		>,
	>
	where
		S: 'async_trait + StatementBuilder,
		'life0: 'async_trait,
		'life1: 'async_trait,
		Self: 'async_trait,
	{
		self.0.query_all(stmt)
	}

	fn support_returning(&self) -> bool {
		self.0.support_returning()
	}

	fn is_mock_connection(&self) -> bool {
		self.0.is_mock_connection()
	}
}
