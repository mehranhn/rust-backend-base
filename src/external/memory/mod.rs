use tokio_util::sync::CancellationToken;

use crate::{app::AppConfig, external::memory::types::ExMTtlValue};

pub mod implementations;

pub mod errors;
pub mod types;

pub trait ExMemoryBase: Send + Sync + 'static {
	fn get(&self, key: &str)
	-> impl Future<Output = Result<String, errors::ErrExMemoryGet>> + Send;

	fn set(
		&self, key: &str, value: String, ttl: ExMTtlValue,
	) -> impl Future<Output = Result<(), errors::ErrExMemorySet>> + Send;

	fn upsert(
		&self, key: &str, value: String, ttl: ExMTtlValue,
	) -> impl Future<Output = Result<Option<String>, errors::ErrExMemoryUpsert>> + Send;

	fn update_ttl(
		&self, key: &str, ttl: ExMTtlValue,
	) -> impl Future<Output = Result<(), errors::ErrExMemoryUpdateTtl>> + Send;

	fn delete(
		&self, key: &str,
	) -> impl Future<Output = Result<usize, errors::ErrExMemoryDelete>> + Send;

	fn delete_many(
		&self, keys: &[&str],
	) -> impl Future<Output = Result<usize, errors::ErrExMemoryDelete>> + Send;

	fn get_i32(
		&self, key: &str,
	) -> impl Future<Output = Result<i32, errors::ErrExMemoryGet>> + Send;

	fn set_i32(
		&self, key: &str, value: i32, ttl: ExMTtlValue,
	) -> impl Future<Output = Result<(), errors::ErrExMemorySet>> + Send;

	fn upsert_i32(
		&self, key: &str, value: i32, ttl: ExMTtlValue,
	) -> impl Future<Output = Result<Option<i32>, errors::ErrExMemoryUpsert>> + Send;

	fn update_ttl_i32(
		&self, key: &str, ttl: ExMTtlValue,
	) -> impl Future<Output = Result<(), errors::ErrExMemoryUpdateTtl>> + Send;

	fn fetch_add_or_set_i32(
		&self, key: &str, value: i32, ttl: Option<ExMTtlValue>,
	) -> impl Future<Output = Result<i32, errors::ErrExMemoryFetchAddOrSet>> + Send;

	fn delete_i32(
		&self, key: &str,
	) -> impl Future<Output = Result<usize, errors::ErrExMemoryDelete>> + Send;

	fn delete_many_i32(
		&self, keys: &[&str],
	) -> impl Future<Output = Result<usize, errors::ErrExMemoryDelete>> + Send;
}

pub trait ExMemory: ExMemoryBase {
	fn run_background_workers(
		&'static self, config: &AppConfig, cancellation_token: CancellationToken,
	);
}
