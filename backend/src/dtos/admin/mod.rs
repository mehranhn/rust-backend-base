mod create;
mod update;

use serde::Serialize;
use serde_with::serde_as;
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

pub use create::AdminCreateDto;
pub use update::AdminUpdateDto;

#[serde_as]
#[derive(Debug, Clone, Serialize, ToSchema, custom_macros::Sortable)]
pub struct AdminDto {
	pub id: Uuid,

	#[serde(with = "time::serde::rfc3339")]
	pub created_at: OffsetDateTime,

	pub deleted_at: Option<OffsetDateTime>,
	pub username: String,
	pub test_account_exp_in_days: u64,
	pub test_account_rx_tx_limit: u64,
	pub delete_inactive_customers_after_days: Option<u64>,
}
