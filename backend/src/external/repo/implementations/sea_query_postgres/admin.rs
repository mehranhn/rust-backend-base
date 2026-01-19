use std::marker::Send;

use sea_query::{Asterisk, Expr, ExprTrait, Func, PostgresQueryBuilder, Query};
use sea_query_sqlx::SqlxBinder;
use uuid::Uuid;

use crate::{
	dtos::{
		AdminCreateDto, AdminDto, AdminDtoSortColumns, AdminUpdateDto, NullUndefinedValue, PaginatedResult, PaginationFilterWithSearchOrder
	},
	external::repo::{
		errors::{
			ErrExRepoAdminCreate, ErrExRepoAdminDelete, ErrExRepoAdminGetById, ErrExRepoAdminUpdate,
		}, implementations::sea_query_postgres::{
			helpers::CountHelper,
			models::{User, UserIden},
			types::Roles,
		}, ExRepoAdmin
	},
	app::errors::ErrServerError,
};

use super::utils::ExRepoImplSeaQueryHandle;

impl<T: ExRepoImplSeaQueryHandle + Send> ExRepoAdmin for T {
	async fn admin_get_list(
		&mut self, filter: PaginationFilterWithSearchOrder<AdminDtoSortColumns>,
	) -> Result<PaginatedResult<AdminDto>, ErrServerError> {
		let mut q = Query::select();
		q.from(UserIden::Table)
			.and_where(Expr::col((UserIden::Table, UserIden::Role)).eq(Roles::Admin.into_expr()))
			.and_where(Expr::col((UserIden::Table, UserIden::DeletedAt)).is_null());

		let (sql, values) = q
			.clone()
			.column((UserIden::Table, Asterisk))
			.offset(filter.skip())
			.limit(filter.take())
			.build_sqlx(PostgresQueryBuilder);

		let data_res = sqlx::query_as_with::<_, User, _>(&sql, values)
			.fetch_all(self.raw_connection())
			.await?;

		let (sql, values) = q
			.expr(Func::count(Expr::col(UserIden::Id)))
			.build_sqlx(PostgresQueryBuilder);

		let count_res = sqlx::query_as_with::<_, CountHelper, _>(&sql, values)
			.fetch_one(self.raw_connection())
			.await?;

		Ok(PaginatedResult::new(
			data_res.into_iter().map(|r| r.into()).collect(),
			count_res.count as u64,
		))
	}

	async fn admin_get_by_id(&mut self, id: Uuid) -> Result<AdminDto, ErrExRepoAdminGetById> {
		let (sql, values) = Query::select()
			.from(UserIden::Table)
			.column((UserIden::Table, Asterisk))
			.and_where(Expr::col((UserIden::Table, UserIden::Id)).eq(id))
			.and_where(Expr::col((UserIden::Table, UserIden::Role)).eq(Roles::Admin.into_expr()))
			.and_where(Expr::col((UserIden::Table, UserIden::DeletedAt)).is_null())
			.build_sqlx(PostgresQueryBuilder);

		let res = sqlx::query_as_with::<_, User, _>(&sql, values)
			.fetch_one(self.raw_connection())
			.await?;

		Ok(res.into())
	}

	async fn admin_create(
		&mut self, id: uuid::Uuid, dto: AdminCreateDto<Vec<u8>>,
	) -> Result<(), ErrExRepoAdminCreate> {
		let (sql, values) = Query::insert()
			.into_table(UserIden::Table)
			.columns([
				UserIden::Id,
				UserIden::Role,
				UserIden::Username,
				UserIden::HashedPassword,
				UserIden::TestAccountExpInDays,
				UserIden::TestAccountRxTxLimit,
				UserIden::DeleteInactiveCustomersAfterDays,
			])
			.values([
				id.into(),
				Roles::Admin.into(),
				dto.username.into_inner().into(),
				dto.password.into(),
				dto.test_account_exp_in_days.into(),
				dto.test_account_rx_tx_limit.into(),
				dto.delete_inactive_customers_after_days.into(),
			])?
			.build_sqlx(PostgresQueryBuilder);

		sqlx::query_with(&sql, values)
			.execute(self.raw_connection())
			.await?;

		Ok(())
	}

	async fn admin_update(
		&mut self, id: Uuid, dto: AdminUpdateDto,
	) -> Result<(), ErrExRepoAdminUpdate> {
		let mut q = Query::update();

		q.table(UserIden::Table)
			.and_where(Expr::col(UserIden::Id).eq(id))
			.and_where(Expr::col((UserIden::Table, UserIden::DeletedAt)).is_null());

		if let Some(v) = dto.username {
			q.value(UserIden::Username, v);
		}

		if let Some(v) = dto.test_account_exp_in_days {
			q.value(UserIden::TestAccountExpInDays, v);
		}

		if let Some(v) = dto.test_account_rx_tx_limit {
			q.value(UserIden::TestAccountRxTxLimit, v);
		}

		match dto.delete_inactive_customers_after_days {
			NullUndefinedValue::Some(v) => {
				q.value(UserIden::DeleteInactiveCustomersAfterDays, v);
			},
			NullUndefinedValue::Null => {
				q.value(
					UserIden::DeleteInactiveCustomersAfterDays,
					Option::<i64>::None,
				);
			},
			NullUndefinedValue::Undefined => {},
		};

		let (sql, values) = q.build_sqlx(PostgresQueryBuilder);

		let res = sqlx::query_with(&sql, values)
			.execute(self.raw_connection())
			.await?;

		if res.rows_affected() == 0 {
			return Err(ErrExRepoAdminUpdate::NotFound);
		}

		Ok(())
	}

	async fn admin_delete(&mut self, id: Uuid) -> Result<(), ErrExRepoAdminDelete> {
		let (sql, values) = Query::update()
			.table(UserIden::Table)
			.value(UserIden::DeletedAt, Expr::current_timestamp())
			.and_where(Expr::col(UserIden::Id).eq(id))
			.and_where(Expr::col((UserIden::Table, UserIden::DeletedAt)).is_null())
			.build_sqlx(PostgresQueryBuilder);

		let res = sqlx::query_with(&sql, values)
			.execute(self.raw_connection())
			.await?;

		if res.rows_affected() == 0 {
			return Err(ErrExRepoAdminDelete::NotFound);
		}

		Ok(())
	}
}
