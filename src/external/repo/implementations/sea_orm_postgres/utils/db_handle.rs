use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbBackend, DbErr, ExecResult, QueryResult, Statement, StatementBuilder};

use crate::{
	app::errors::ErrServerError,
	external::repo::{
		ExRepoConnection, ExRepoLogic, ExRepoTranactional, ExRepoTx,
		implementations::sea_orm_postgres::utils::{
			DbHandleConnection, transaction::DbHandleTransaction,
		},
	},
};

pub trait DbHandleInner: ConnectionTrait + Send {
	fn tx<'a>(
		&'a mut self,
	) -> impl Future<Output = Result<DbHandle<DbHandleTransaction<'a>>, DbErr>> + Send;
}

pub struct DbHandle<T: DbHandleInner>(T);

impl DbHandle<DbHandleConnection> {
	pub fn new_connection(c: DatabaseConnection) -> Self {
		DbHandle(DbHandleConnection::new(c))
	}
}

impl DbHandle<DbHandleTransaction<'static>> {
	pub fn new_transaction(c: DatabaseTransaction) -> Self {
		DbHandle(DbHandleTransaction::new(c))
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


impl<T: DbHandleInner> ConnectionTrait for DbHandle<T> {
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
