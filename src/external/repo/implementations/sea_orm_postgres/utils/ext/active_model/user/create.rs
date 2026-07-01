use sea_orm::{ActiveValue, IntoActiveModel};

use crate::{
	dtos::AdminCreateDto,
	external::repo::implementations::sea_orm_postgres::{models::user, types::roles::Roles},
	utils::generate_uuid,
};

impl IntoActiveModel<user::ActiveModel> for AdminCreateDto<Vec<u8>> {
	fn into_active_model(self) -> user::ActiveModel {
		user::ActiveModel {
			id: ActiveValue::Set(generate_uuid()),
			role: ActiveValue::Set(Roles::Admin),
			username: ActiveValue::Set(self.username.into_inner()),
			hashed_password: ActiveValue::Set(self.password),
			phone: ActiveValue::Set(self.phone.map(|p| p.into_inner())),
			email: ActiveValue::Set(self.email.map(|e| e.into_inner())),
			..Default::default()
		}
	}
}
