use sea_orm::entity::prelude::*;
use time::OffsetDateTime;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
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

	#[sea_orm(column_type = "Text")]
	pub content: String,

    #[sea_orm(has_many)]
	pub comments: HasMany<super::comment::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
