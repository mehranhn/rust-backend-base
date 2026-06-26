use crate::{enums::Roles, permission::Permissions};

impl Roles {
	pub const fn has_permission(&self, permission: Permissions) -> bool {
		match self {
			Roles::Admin => matches!(
				permission,
				Permissions::AdminRead
					| Permissions::AdminCreate
					| Permissions::AdminUpdate
					| Permissions::AdminDelete
			),
			Roles::User => false,
		}
	}

	pub fn has_permission_any(&self, permissions: &[Permissions]) -> bool {
		for p in permissions {
			if self.has_permission(*p) {
				return true;
			}
		}

		false
	}

	pub fn has_permission_all(&self, permissions: &[Permissions]) -> bool {
		for p in permissions {
			if !self.has_permission(*p) {
				return false;
			}
		}

		true
	}
}
