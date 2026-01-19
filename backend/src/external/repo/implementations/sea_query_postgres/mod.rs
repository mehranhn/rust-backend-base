use sea_query::{OnConflict, PostgresQueryBuilder, Query};
use sea_query_sqlx::SqlxBinder;
use sqlx::PgPool;

use crate::{
	dtos::SeedDto,
	external::repo::{
		ExRepo,
		implementations::sea_query_postgres::{
			models::UserIden,
			types::Roles,
			utils::{ExRepoImplSeaQueryPgConnection, ExRepoImplSeaQueryPgTx},
		},
	},
	app::errors::ErrServerError,
	utils::generate_uuid,
};

mod admin;
mod auth;
mod helpers;
mod models;
mod types;
mod utils;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./src/external/repo/implementations/sea_query_postgres/migrations");

pub struct ExRepoImplSeaQueryPg {
	pool: PgPool,
}

impl ExRepoImplSeaQueryPg {
	pub async fn new(database_url: &str) -> Result<Self, sqlx::error::Error> {
		let pool = PgPool::connect(database_url).await?;
		Ok(Self { pool })
	}
}

impl ExRepo for ExRepoImplSeaQueryPg {
	type Connection = ExRepoImplSeaQueryPgConnection;
	type Transaction = ExRepoImplSeaQueryPgTx<'static>;

	async fn connection(&self) -> Result<Self::Connection, ErrServerError> {
		let c = self.pool.acquire().await?;
		Ok(ExRepoImplSeaQueryPgConnection::new(c))
	}

	async fn transaction(&self) -> Result<Self::Transaction, ErrServerError> {
		let tx = self.pool.begin().await?;
		Ok(ExRepoImplSeaQueryPgTx::new(tx))
	}

	async fn run_migrations(&self) -> Result<(), ErrServerError> {
		MIGRATOR.run(&self.pool).await?;
		Ok(())
	}

	async fn seed(&self, dto: SeedDto<'_>) -> Result<(), ErrServerError> {
		let (sql, values) = Query::insert()
			.into_table(UserIden::Table)
			.columns([
				UserIden::Id,
				UserIden::Username,
				UserIden::HashedPassword,
				UserIden::Role,
			])
			.values([
				generate_uuid().into(),
				dto.super_admin_username.into(),
				dto.super_admin_hashed_password.as_slice().into(),
				Roles::Admin.into(),
			])?
			.on_conflict(
				OnConflict::column(UserIden::Username)
					.update_columns([UserIden::Username, UserIden::Role])
					.to_owned(),
			)
			.build_sqlx(PostgresQueryBuilder);

		sqlx::query_with(&sql, values).execute(&self.pool).await?;

		Ok(())
	}
}
