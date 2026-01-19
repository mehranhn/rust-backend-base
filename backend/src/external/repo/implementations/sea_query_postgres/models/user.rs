use sea_query::enum_def;
use sqlx::prelude::FromRow;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::dtos::{AdminDto, UserLoginDto};

use super::super::types::Roles;

#[enum_def(table_name = "users")]
#[derive(Debug, FromRow)]
pub struct User {
	id: Uuid,
	created_at: PrimitiveDateTime,
	deleted_at: Option<PrimitiveDateTime>,
	role: Roles,
	username: String,
	hashed_password: Vec<u8>,
	test_account_exp_in_days: i64,
	test_account_rx_tx_limit: i64,
	delete_inactive_customers_after_days: Option<i64>,
}

impl Into<UserLoginDto> for User {
	fn into(self) -> UserLoginDto {
		UserLoginDto {
			id: self.id,
			username: self.username,
			hashed_password: self.hashed_password,
			role: self.role.into(),
		}
	}
}

impl Into<AdminDto> for User {
	fn into(self) -> AdminDto {
		AdminDto {
			id: self.id,
			created_at: self.created_at.assume_utc(),
			deleted_at: self.deleted_at.map(|d| d.assume_utc()),
			username: self.username,
			test_account_exp_in_days: self.test_account_exp_in_days as u64,
			test_account_rx_tx_limit: self.test_account_rx_tx_limit as u64,
			delete_inactive_customers_after_days: self
				.delete_inactive_customers_after_days
				.map(|d| d as u64),
		}
	}
}
