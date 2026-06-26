use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, ToSchema)]
pub enum Roles {
	Admin,
	User,
}

