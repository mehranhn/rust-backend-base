use sqlx::{FromRow, Row, postgres::PgRow};

#[derive(Debug)]
pub struct CountHelper {
	pub count: i64,
}

impl<'r> FromRow<'r, PgRow> for CountHelper {
	fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
		Ok(Self {
			count: row.try_get::<'_, i64, _>(0)?,
		})
	}
}
