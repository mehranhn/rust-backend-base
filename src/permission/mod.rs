mod roles;

use custom_macros::GeneratePermissions;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema, GeneratePermissions)]
#[prefix = "Perm"]
#[repr(u64)]
pub enum Permissions {
	AdminRead,
	AdminCreate,
	AdminUpdate,
	AdminDelete,
}

pub trait Permission {
	fn permission() -> Permissions;
}
