use serde::Deserialize;
use utoipa::ToSchema;

use crate::dtos::NullUndefinedValue;

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AdminUpdateDto {
	#[schema(required = false, value_type = String)]
	pub username: Option<String>,

	#[schema(required = false, value_type = u64)]
	pub test_account_exp_in_days: Option<u64>,

	#[schema(required = false, value_type = u64)]
	pub test_account_rx_tx_limit: Option<u64>,

	#[schema(value_type = Option<u64>)]
	#[serde(default)]
	pub delete_inactive_customers_after_days: NullUndefinedValue<u64>,
}
