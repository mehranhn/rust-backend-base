mod create;
mod update;

use serde::Serialize;
use serde_with::serde_as;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

pub use create::UserCreateDto;
pub use update::UserUpdateDto;

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct UserDto {
	pub id: Uuid,

	#[serde(with = "time::serde::rfc3339")]
	pub created_at: OffsetDateTime,

	#[serde(with = "time::serde::rfc3339")]
	pub updated_at: OffsetDateTime,

	pub email: String,
	pub phone: String,
	pub first_name: String,
	pub last_name: String,
}
