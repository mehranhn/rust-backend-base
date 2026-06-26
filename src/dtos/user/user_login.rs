use uuid::Uuid;

use crate::enums::Roles;

#[derive(Debug, Clone)]
pub struct UserLoginDto {
	pub id: Uuid,
	pub username: String,
	pub hashed_password: Vec<u8>,
	pub role: Roles,
}
