use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UserUpdateDto {
	pub first_name: Option<String>,
	pub last_name: Option<String>,
}
