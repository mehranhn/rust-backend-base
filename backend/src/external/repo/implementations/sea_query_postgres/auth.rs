use std::marker::Send;

use sea_query::{Asterisk, Expr, ExprTrait, PostgresQueryBuilder, Query};
use sea_query_sqlx::SqlxBinder;
use sqlx::FromRow;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use crate::{
	dtos::{AuthData, UserLoginDto},
	external::repo::{
		auth::ExRepoAuth,
		errors::{ErrExRepoAuthRenewSession, ErrExRepoUserGetUserByUsername},
		implementations::sea_query_postgres::{
			models::{SessionIden, User, UserIden},
			types::Roles,
		},
	},
	app::errors::ErrServerError,
};

use super::utils::ExRepoImplSeaQueryHandle;

impl<T: ExRepoImplSeaQueryHandle + Send> ExRepoAuth for T {
	async fn get_user_by_username_for_login(
		&mut self, username: &str,
	) -> Result<UserLoginDto, ErrExRepoUserGetUserByUsername> {
		let (sql, values) = Query::select()
			.from(UserIden::Table)
			.column((UserIden::Table, Asterisk))
			.and_where(Expr::col((UserIden::Table, UserIden::Username)).eq(username))
			.and_where(Expr::col((UserIden::Table, UserIden::DeletedAt)).is_null())
			.build_sqlx(PostgresQueryBuilder);

		let res = sqlx::query_as_with::<_, User, _>(&sql, values)
			.fetch_one(self.raw_connection())
			.await?;

		Ok(res.into())
	}

	async fn session_get_auth(
		&mut self, session_id: Uuid,
	) -> Result<AuthData, ErrExRepoAuthRenewSession> {
		#[derive(Debug, FromRow)]
		pub struct Tmp {
			id: Uuid,
			role: Roles,
			username: String,
			session_id: Uuid,
			expire_at: Option<PrimitiveDateTime>,
		}

		let (sql, values) = Query::select()
			.from(SessionIden::Table)
			.column((UserIden::Table, UserIden::Id))
			.column((UserIden::Table, UserIden::Role))
			.column((UserIden::Table, UserIden::Username))
			.expr_as(
				Expr::col((SessionIden::Table, SessionIden::Id)),
				"session_id",
			)
			.column((SessionIden::Table, SessionIden::ExpireAt))
			.inner_join(
				UserIden::Table,
				Expr::col((SessionIden::Table, SessionIden::UserId))
					.equals((UserIden::Table, UserIden::Id)),
			)
			.and_where(Expr::col((SessionIden::Table, SessionIden::Id)).eq(session_id))
			.and_where(Expr::col((SessionIden::Table, SessionIden::DeletedAt)).is_null())
			.build_sqlx(PostgresQueryBuilder);

		let res = sqlx::query_as_with::<_, Tmp, _>(&sql, values)
			.fetch_one(self.raw_connection())
			.await?;

		Ok(AuthData {
			user_id: res.id,
			session_id: res.session_id,
			role: res.role.into(),
			username: res.username,
			expire_at: res.expire_at.map(|e| e.assume_utc()),
		})
	}

	async fn session_create(
		&mut self, session_id: Uuid, user_id: Uuid, expire_at: Option<OffsetDateTime>,
	) -> Result<(), ErrServerError> {
		let exp: Option<PrimitiveDateTime> = expire_at.map(|e| PrimitiveDateTime::new(e.date(), e.time()));
		let (sql, values) = Query::insert()
			.into_table(SessionIden::Table)
			.columns([SessionIden::Id, SessionIden::UserId, SessionIden::ExpireAt])
			.values([session_id.into(), user_id.into(), exp.into()])?
			.build_sqlx(PostgresQueryBuilder);

		sqlx::query_with(&sql, values)
			.execute(self.raw_connection())
			.await?;

		Ok(())
	}

	async fn session_renew(&mut self, session_id: Uuid) -> Result<(), ErrExRepoAuthRenewSession> {
		let (sql, values) = Query::update()
			.table(SessionIden::Table)
			.value(SessionIden::LastAccess, Expr::current_timestamp())
			.and_where(Expr::col(SessionIden::Id).eq(session_id))
			.and_where(Expr::col((SessionIden::Table, SessionIden::DeletedAt)).is_null())
			.build_sqlx(PostgresQueryBuilder);

		sqlx::query_with(&sql, values)
			.execute(self.raw_connection())
			.await?;

		Ok(())
	}

	async fn session_logout(&mut self, session_id: Uuid) -> Result<(), ErrServerError> {
		let (sql, values) = Query::update()
			.table(SessionIden::Table)
			.value(SessionIden::DeletedAt, Expr::current_timestamp())
			.and_where(Expr::col(SessionIden::Id).eq(session_id))
			.and_where(Expr::col((SessionIden::Table, SessionIden::DeletedAt)).is_null())
			.build_sqlx(PostgresQueryBuilder);

		sqlx::query_with(&sql, values)
			.execute(self.raw_connection())
			.await?;

		Ok(())
	}
}
