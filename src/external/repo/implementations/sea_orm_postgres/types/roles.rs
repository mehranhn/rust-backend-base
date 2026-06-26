use sea_orm::{DeriveActiveEnum, EnumIter};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[repr(i32)]
pub enum Roles {
	Admin = 0,
	User = 1,
}

impl From<crate::enums::Roles> for Roles {
	fn from(value: crate::enums::Roles) -> Self {
		match value {
			crate::enums::Roles::Admin => Roles::Admin,
			crate::enums::Roles::User => Roles::User,
		}
	}
}

impl Into<crate::enums::Roles> for Roles {
	fn into(self) -> crate::enums::Roles {
		match self {
			Roles::Admin => crate::enums::Roles::Admin,
			Roles::User => crate::enums::Roles::User,
		}
	}
}
