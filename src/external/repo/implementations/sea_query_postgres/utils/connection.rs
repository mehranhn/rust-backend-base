use std::ops::DerefMut;

use sqlx::{Connection, PgConnection, Postgres, pool::PoolConnection};

use super::db_handle::{DbHandle, DbHandleInner};

#[derive(Debug)]
pub struct DbHandleConnection(PoolConnection<Postgres>);

impl DbHandleConnection {
	pub fn new(c: PoolConnection<Postgres>) -> Self {
		Self(c)
	}
}

impl DbHandleInner for DbHandleConnection {
	fn raw_connection(&mut self) -> &mut PgConnection {
		self.0.deref_mut()
	}

	async fn tx<'a>(
		&'a mut self,
	) -> Result<DbHandle<super::transaction::DbHandleTransaction<'a>>, sqlx::Error> {
		let tx = self.0.begin().await?;
		Ok(DbHandle::new_transaction(tx))
	}
}

// impl<'c> Executor<'c> for DbHandleConnection {
// 	type Database = Postgres;
//
// 	fn fetch_many<'e, 'q: 'e, E>(
// 		mut self, query: E,
// 	) -> futures::stream::BoxStream<
// 		'e,
// 		Result<
// 			sqlx::Either<
// 				<Self::Database as sqlx::Database>::QueryResult,
// 				<Self::Database as sqlx::Database>::Row,
// 			>,
// 			sqlx::Error,
// 		>,
// 	>
// 	where
// 		'c: 'e,
// 		E: 'q + sqlx::Execute<'q, Self::Database>,
// 	{
// 		self.0.fetch_many(query)
// 	}
//
// 	fn fetch_optional<'e, 'q: 'e, E>(
// 		mut self, query: E,
// 	) -> futures::future::BoxFuture<
// 		'e,
// 		Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>,
// 	>
// 	where
// 		'c: 'e,
// 		E: 'q + sqlx::Execute<'q, Self::Database>,
// 	{
// 		self.0.fetch_optional(query)
// 	}
//
// 	fn prepare_with<'e>(
// 		mut self, sql: sqlx::SqlStr, parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
// 	) -> futures::future::BoxFuture<
// 		'e,
// 		Result<<Self::Database as sqlx::Database>::Statement, sqlx::Error>,
// 	>
// 	where
// 		'c: 'e,
// 	{
// 		self.0.prepare_with(sql, parameters)
// 	}
//
// 	fn describe<'e>(
// 		mut self, sql: sqlx::SqlStr,
// 	) -> futures::future::BoxFuture<'e, Result<Describe<Self::Database>, sqlx::Error>>
// 	where
// 		'c: 'e,
// 	{
// 		self.0.describe(sql)
// 	}
//
// 	fn execute<'e, 'q: 'e, E>(
// 		mut self, query: E,
// 	) -> futures::future::BoxFuture<
// 		'e,
// 		Result<<Self::Database as sqlx::Database>::QueryResult, sqlx::Error>,
// 	>
// 	where
// 		'c: 'e,
// 		E: 'q + sqlx::Execute<'q, Self::Database>,
// 	{
// 		self.0.execute(query)
// 	}
//
// 	fn execute_many<'e, 'q: 'e, E>(
// 		mut self, query: E,
// 	) -> futures::stream::BoxStream<
// 		'e,
// 		Result<<Self::Database as sqlx::Database>::QueryResult, sqlx::Error>,
// 	>
// 	where
// 		'c: 'e,
// 		E: 'q + sqlx::Execute<'q, Self::Database>,
// 	{
// 		self.0.execute_many(query)
// 	}
//
// 	fn fetch<'e, 'q: 'e, E>(
// 		mut self, query: E,
// 	) -> futures::stream::BoxStream<'e, Result<<Self::Database as sqlx::Database>::Row, sqlx::Error>>
// 	where
// 		'c: 'e,
// 		E: 'q + sqlx::Execute<'q, Self::Database>,
// 	{
// 		self.0.fetch(query)
// 	}
//
// 	fn fetch_all<'e, 'q: 'e, E>(
// 		mut self, query: E,
// 	) -> futures::future::BoxFuture<
// 		'e,
// 		Result<Vec<<Self::Database as sqlx::Database>::Row>, sqlx::Error>,
// 	>
// 	where
// 		'c: 'e,
// 		E: 'q + sqlx::Execute<'q, Self::Database>,
// 	{
// 		self.0.fetch_all(query)
// 	}
//
// 	fn fetch_one<'e, 'q: 'e, E>(
// 		mut self, query: E,
// 	) -> futures::future::BoxFuture<'e, Result<<Self::Database as sqlx::Database>::Row, sqlx::Error>>
// 	where
// 		'c: 'e,
// 		E: 'q + sqlx::Execute<'q, Self::Database>,
// 	{
// 		self.0.fetch_one(query)
// 	}
//
// 	fn prepare<'e>(
// 		mut self, query: sqlx::SqlStr,
// 	) -> futures::future::BoxFuture<
// 		'e,
// 		Result<<Self::Database as sqlx::Database>::Statement, sqlx::Error>,
// 	>
// 	where
// 		'c: 'e,
// 	{
// 		self.0.prepare(query)
// 	}
// }
