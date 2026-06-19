use serde::Deserialize;
use utoipa::ToSchema;

use crate::validators::{StringVPassword, StringVUsername};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct LoginDto {
	pub username: StringVUsername,
	pub password: StringVPassword,
}
