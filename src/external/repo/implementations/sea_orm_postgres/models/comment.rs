use sea_orm::entity::prelude::*;
use time::OffsetDateTime;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "comments")]
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

	pub post_id: Uuid,

	#[sea_orm(belongs_to, from = "post_id", to = "id")]
	pub post: HasOne<super::post::Entity>,

	#[sea_orm(column_type = "Text")]
	pub content: String,

	pub reply_to_id: Option<Uuid>,

	#[sea_orm(
		self_ref,
		relation_enum = "ReplyTo",
		relation_reverse = "Replies",
		from = "reply_to_id",
		to = "id"
	)]
	pub reply_to: HasOne<Entity>,

    #[sea_orm(self_ref, relation_enum = "Replies", relation_reverse = "ReplyTo")]
	pub replies: HasMany<Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
