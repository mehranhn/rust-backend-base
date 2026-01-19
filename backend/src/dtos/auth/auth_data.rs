use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::permission::Roles;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, ToSchema)]
pub struct AuthData {
	pub user_id: Uuid,
	pub session_id: Uuid,
	pub role: Roles,
	pub username: String,

	#[serde(with = "time::serde::rfc3339::option")]
	pub expire_at: Option<OffsetDateTime>,
}
