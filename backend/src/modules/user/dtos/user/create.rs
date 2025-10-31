use serde::Deserialize;
use time::PrimitiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct UserCreateDto {
	pub email: String,
	pub phone: String,
	pub first_name: String,
	pub last_name: String,
}
