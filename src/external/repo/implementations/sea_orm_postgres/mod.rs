use sea_orm::{ActiveValue, Database, DatabaseConnection, DbErr, EntityTrait, TransactionTrait};
use sea_orm_migrations::MigratorTrait;
use sea_query::OnConflict;

use crate::{
	app::errors::ErrServerError,
	dtos::SeedDto,
	external::repo::{
		ExRepo,
		implementations::sea_orm_postgres::{
			types::roles::Roles,
			utils::{DbHandle, DbHandleConnection, DbHandleTransaction},
		},
	},
	utils::generate_uuid,
};

mod admin;
mod auth;
mod models;
mod partials;
mod types;
mod utils;

#[allow(dead_code)]
pub struct ExRepoImplSeaOrmPg {
	connection: DatabaseConnection,
}

impl ExRepoImplSeaOrmPg {
	#[allow(dead_code)]
	pub async fn new(database_url: &str) -> Result<Self, DbErr> {
		let connection = Database::connect(database_url).await?;
		Ok(Self { connection })
	}
}

impl ExRepo for ExRepoImplSeaOrmPg {
	type Connection = DbHandle<DbHandleConnection>;
	type Transaction = DbHandle<DbHandleTransaction<'static>>;

	async fn connection(&self) -> Result<Self::Connection, ErrServerError> {
		let c = self.connection.clone();
		Ok(DbHandle::new_connection(c))
	}

	async fn transaction(&self) -> Result<Self::Transaction, ErrServerError> {
		let tx = self.connection.begin().await?;
		Ok(DbHandle::new_transaction(tx))
	}

	async fn run_migrations(&self) -> Result<(), ErrServerError> {
		sea_orm_migrations::Migrator::up(&self.connection, None).await?;
		Ok(())
	}

	async fn seed(&self, dto: SeedDto) -> Result<(), ErrServerError> {
		let user = models::user::ActiveModel {
			id: ActiveValue::set(generate_uuid()),
			role: ActiveValue::set(Roles::Admin),
			username: ActiveValue::set(dto.super_admin_username),
			hashed_password: ActiveValue::set(dto.super_admin_hashed_password),
			..Default::default()
		};

		models::user::Entity::insert(user)
			.on_conflict(
				OnConflict::column(models::user::Column::Username)
					.update_columns([models::user::Column::Username, models::user::Column::Role])
					.to_owned(),
			)
			.exec(&self.connection)
			.await?;

		Ok(())
	}
}
