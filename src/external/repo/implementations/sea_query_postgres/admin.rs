use sea_query::{
	Asterisk, Expr, ExprTrait, Func, Order, PostgresQueryBuilder, Query,
	extension::postgres::PgExpr,
};
use sea_query_sqlx::SqlxBinder;
use sqlx::{AssertSqlSafe, SqlSafeStr};
use uuid::Uuid;

use crate::{
	app::errors::ErrServerError,
	dtos::{
		AdminCreateDto, AdminDto, AdminDtoSortColumns, AdminUpdateDto, NullUndefinedValue,
		PaginatedResult, PaginationFilterWithSearchOrder,
	},
	external::repo::{
		ExRepoAdmin,
		errors::{
			ErrExRepoAdminCreate, ErrExRepoAdminDelete, ErrExRepoAdminGetById, ErrExRepoAdminUpdate,
		},
		implementations::sea_query_postgres::{
			helpers::CountHelper,
			models::{User, UserIden},
			types::Roles,
			utils::{DbHandle, DbHandleInner},
		},
	},
};

impl<T: DbHandleInner> ExRepoAdmin for DbHandle<T> {
	async fn admin_get_list(
		&mut self, filter: PaginationFilterWithSearchOrder<AdminDtoSortColumns>,
	) -> Result<PaginatedResult<AdminDto>, ErrServerError> {
		let mut q = Query::select();

		q.from(UserIden::Table)
			.and_where(Expr::col((UserIden::Table, UserIden::Role)).eq(Roles::Admin.into_expr()));

		if filter.search().as_str() != "" {
			q.and_where(
				Expr::col((UserIden::Table, UserIden::Username)).contains(filter.search().as_str()),
			);
		}

		let ord = if filter.order_by_is_asc() {
			Order::Asc
		} else {
			Order::Desc
		};

		if let Some(c) = filter.order_by_column() {
			match c {
				AdminDtoSortColumns::Id => q.order_by(UserIden::Id, ord),
				AdminDtoSortColumns::CreatedAt =>  q.order_by(UserIden::CreatedAt, ord),
				AdminDtoSortColumns::Username =>  q.order_by(UserIden::Username, ord),
				AdminDtoSortColumns::Phone =>  q.order_by(UserIden::Phone, ord),
				AdminDtoSortColumns::Email =>  q.order_by(UserIden::Email, ord),
			};
		}

		let (sql, values) = q
			.clone()
			.column((UserIden::Table, Asterisk))
			.offset(filter.skip())
			.limit(filter.take())
			.build_sqlx(PostgresQueryBuilder);

		let data_res = sqlx::query_as_with::<_, User, _>(AssertSqlSafe(sql).into_sql_str(), values)
			.fetch_all(self.raw_connection())
			.await?;

		let (sql, values) = q
			.expr(Func::count(Expr::col(UserIden::Id)))
			.build_sqlx(PostgresQueryBuilder);

		let count_res =
			sqlx::query_as_with::<_, CountHelper, _>(AssertSqlSafe(sql).into_sql_str(), values)
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
			.build_sqlx(PostgresQueryBuilder);

		let res = sqlx::query_as_with::<_, User, _>(AssertSqlSafe(sql).into_sql_str(), values)
			.fetch_one(self.raw_connection())
			.await?;

		Ok(res.into())
	}

	async fn admin_create(
		&mut self, dto: AdminCreateDto<Vec<u8>>,
	) -> Result<(), ErrExRepoAdminCreate> {
		let (sql, values) = Query::insert()
			.into_table(UserIden::Table)
			.columns([
				UserIden::Id,
				UserIden::Role,
				UserIden::Username,
				UserIden::HashedPassword,
				UserIden::Phone,
				UserIden::Email,
			])
			.values([
				Roles::Admin.into(),
				dto.username.into_inner().into(),
				dto.password.into(),
				dto.phone.map(|p| p.into_inner()).into(),
				dto.email.map(|e| e.into_inner()).into(),
			])?
			.build_sqlx(PostgresQueryBuilder);

		sqlx::query_with(AssertSqlSafe(sql).into_sql_str(), values)
			.execute(self.raw_connection())
			.await?;

		Ok(())
	}

	async fn admin_update(
		&mut self, id: Uuid, dto: AdminUpdateDto,
	) -> Result<(), ErrExRepoAdminUpdate> {
		let mut q = Query::update();

		q.table(UserIden::Table)
			.and_where(Expr::col(UserIden::Id).eq(id));

		if let Some(v) = dto.username {
			q.value(UserIden::Username, v);
		}

		match dto.phone {
			NullUndefinedValue::Some(v) => {
				q.value(UserIden::Phone, v.into_inner());
			},
			NullUndefinedValue::Null => {
				q.value(UserIden::Phone, Option::<String>::None);
			},
			NullUndefinedValue::Undefined => {},
		}

		match dto.email {
			NullUndefinedValue::Some(v) => {
				q.value(UserIden::Email, v.into_inner());
			},
			NullUndefinedValue::Null => {
				q.value(UserIden::Email, Option::<String>::None);
			},
			NullUndefinedValue::Undefined => {},
		}

		let (sql, values) = q.build_sqlx(PostgresQueryBuilder);

		let res = sqlx::query_with(AssertSqlSafe(sql).into_sql_str(), values)
			.execute(self.raw_connection())
			.await?;

		if res.rows_affected() == 0 {
			return Err(ErrExRepoAdminUpdate::NotFound);
		}

		Ok(())
	}

	async fn admin_delete(&mut self, id: Uuid) -> Result<(), ErrExRepoAdminDelete> {
		let (sql, values) = Query::delete()
			.from_table(UserIden::Table)
			.and_where(Expr::col(UserIden::Id).eq(id))
			.build_sqlx(PostgresQueryBuilder);

		let res = sqlx::query_with(AssertSqlSafe(sql).into_sql_str(), values)
			.execute(self.raw_connection())
			.await?;

		if res.rows_affected() == 0 {
			return Err(ErrExRepoAdminDelete::NotFound);
		}

		Ok(())
	}
}
