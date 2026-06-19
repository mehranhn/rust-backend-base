mod connection;
mod transaction;

pub use connection::ExRepoImplSeaQueryPgConnection;
use sqlx::PgConnection;
pub use transaction::ExRepoImplSeaQueryPgTx;

use crate::app::errors::ErrServerError;

pub trait ExRepoImplSeaQueryHandle {
	fn raw_connection(&mut self) -> &mut PgConnection;
	fn tx<'a>(
		&'a mut self,
	) -> impl Future<Output = Result<ExRepoImplSeaQueryPgTx<'a>, ErrServerError>> + Send;
}
