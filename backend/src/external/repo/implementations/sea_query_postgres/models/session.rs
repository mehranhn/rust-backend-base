use sea_query::enum_def;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

#[enum_def(table_name = "sessions")]
pub struct Session {
	id: Uuid,
	user_id: Uuid,
	created_at: OffsetDateTime,
	deleted_at: Option<PrimitiveDateTime>,
	expire_at: Option<PrimitiveDateTime>,
	last_access: OffsetDateTime,
}
