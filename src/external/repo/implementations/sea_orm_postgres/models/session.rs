use sea_orm::entity::prelude::*;
use time::OffsetDateTime;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: Uuid,

	#[sea_orm(default_value = "CURRENT_TIMESTAMP")]
	pub created_at: OffsetDateTime,

	#[sea_orm(default_value = "CURRENT_TIMESTAMP")]
	pub updated_at: OffsetDateTime,

	pub user_id: Uuid,

	#[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,

	pub expire_at: Option<OffsetDateTime>,

	pub last_access: OffsetDateTime,
}

impl ActiveModelBehavior for ActiveModel {}
