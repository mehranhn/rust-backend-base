use sea_orm::{ActiveValue, IntoActiveModel};

use crate::{
	dtos::{AdminUpdateDto, NullUndefinedValue},
	external::repo::implementations::sea_orm_postgres::models::user,
};

impl IntoActiveModel<user::ActiveModel> for AdminUpdateDto {
	fn into_active_model(self) -> user::ActiveModel {
		let mut active_model = user::ActiveModel {
			..Default::default()
		};

		active_model.username = match self.username {
			Some(u) => ActiveValue::Set(u),
			_ => ActiveValue::NotSet,
		};

		active_model.phone = match self.phone {
			NullUndefinedValue::Some(p) => ActiveValue::Set(Some(p.into_inner())),
			NullUndefinedValue::Null => ActiveValue::Set(None),
			NullUndefinedValue::Undefined => ActiveValue::NotSet,
		};

		active_model.email = match self.email {
			NullUndefinedValue::Some(e) => ActiveValue::Set(Some(e.into_inner())),
			NullUndefinedValue::Null => ActiveValue::Set(None),
			NullUndefinedValue::Undefined => ActiveValue::NotSet,
		};

		active_model
	}
}
