use sea_query::{Expr, ExprTrait};

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "roles")]
#[sqlx(rename_all = "UPPERCASE")]
pub enum Roles {
	Admin,
	User,
}

impl Into<&str> for Roles {
	fn into(self) -> &'static str {
		match self {
			Roles::Admin => "ADMIN",
			Roles::User => "USER",
		}
	}
}

impl Into<Expr> for Roles {
	fn into(self) -> Expr {
		Into::<&str>::into(self).as_enum("roles")
	}
}

impl Roles {
	pub fn into_expr(&self) -> Expr {
		(*self).into()
	}
}

impl From<crate::enums::Roles> for Roles {
	fn from(value: crate::enums::Roles) -> Self {
		match value {
			crate::enums::Roles::Admin => Self::Admin,
			crate::enums::Roles::User => Self::User,
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
