use sea_query::{Expr, ExprTrait};

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "roles")]
#[sqlx(rename_all = "UPPERCASE")]
pub enum Roles {
	Admin,
	Salesmen,
}

impl Into<&str> for Roles {
	fn into(self) -> &'static str {
		match self {
			Roles::Admin => "ADMIN",
			Roles::Salesmen => "SALESMEN",
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

impl From<crate::permission::Roles> for Roles {
	fn from(value: crate::permission::Roles) -> Self {
		match value {
			crate::permission::Roles::Admin => Self::Admin,
			crate::permission::Roles::Salesmen => Self::Salesmen,
		}
	}
}

impl Into<crate::permission::Roles> for Roles {
	fn into(self) -> crate::permission::Roles {
		match self {
			Roles::Admin => crate::permission::Roles::Admin,
			Roles::Salesmen => crate::permission::Roles::Salesmen,
		}
	}
}
