use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
	utils::hash_password,
	validators::{StringVPassword, StringVUsername},
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AdminCreateDto<T> {
	pub username: StringVUsername,
	pub password: T,
	pub test_account_exp_in_days: u64,
	pub test_account_rx_tx_limit: u64,
	pub delete_inactive_customers_after_days: Option<u64>,
}

impl AdminCreateDto<StringVPassword> {
	pub fn into_hashed(self) -> AdminCreateDto<Vec<u8>> {
		let hashed_password = hash_password(&self.username, &self.password);
		AdminCreateDto {
			username: self.username,
			password: hashed_password,
			test_account_exp_in_days: self.test_account_exp_in_days,
			test_account_rx_tx_limit: self.test_account_rx_tx_limit,
			delete_inactive_customers_after_days: self.delete_inactive_customers_after_days,
		}
	}
}
