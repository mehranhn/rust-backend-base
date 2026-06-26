mod hash_password;
mod jwt;

pub use hash_password::hash_password;
pub use hash_password::check_password;
pub use jwt::*;

use uuid::Uuid;

pub fn generate_uuid() -> Uuid {
	Uuid::now_v7()
}
