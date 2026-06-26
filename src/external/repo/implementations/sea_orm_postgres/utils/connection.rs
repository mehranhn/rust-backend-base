use sea_orm::{
	ConnectionTrait, DatabaseConnection, DbBackend, DbErr, ExecResult, QueryResult, Statement,
	StatementBuilder, TransactionTrait,
};

use super::db_handle::{DbHandle, DbHandleInner};

pub struct DbHandleConnection(DatabaseConnection);

impl DbHandleConnection {
	pub fn new(c: DatabaseConnection) -> Self {
		Self(c)
	}
}

impl DbHandleInner for DbHandleConnection {
    async fn tx<'a>(&'a mut self) -> Result<super::DbHandle<super::transaction::DbHandleTransaction<'a>>, DbErr> {
		let tx = self.0.begin().await?;
		Ok(DbHandle::new_transaction(tx))
    }
}

impl ConnectionTrait for DbHandleConnection {
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
