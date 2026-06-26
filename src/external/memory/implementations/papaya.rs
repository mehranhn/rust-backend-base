use std::{
	future::ready,
	sync::atomic::{AtomicI32, AtomicU64, Ordering},
	time::Duration,
};

use futures::FutureExt;
use papaya::HashMap;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::{
	app::AppConfig,
	external::memory::{
		ExMemory, ExMemoryBase,
		errors::{
			ErrExMemoryDelete, ErrExMemoryFetchAddOrSet, ErrExMemoryGet, ErrExMemorySet,
			ErrExMemoryUpdateTtl, ErrExMemoryUpsert,
		},
		types::ExMTtlValue,
	},
};

struct WithTtl<T: Send + Sync + 'static> {
	ttl: AtomicU64,
	data: T,
}

impl<T: Send + Sync + 'static> WithTtl<T> {
	pub fn new(data: T, ttl: ExMTtlValue) -> Self {
		let t: u64 = Self::ttl_value_to_u64(ttl);

		Self {
			ttl: AtomicU64::new(t),
			data,
		}
	}

	fn ttl_value_to_u64(ttl: ExMTtlValue) -> u64 {
		match ttl {
			ExMTtlValue::Duration(d) => {
				let now = time::OffsetDateTime::now_utc();
				let then = now + d;
				then.unix_timestamp() as u64
			},
			ExMTtlValue::AtTime(t) => t.assume_utc().unix_timestamp() as u64,
			ExMTtlValue::NoExp => 0,
		}
	}

	pub fn is_expired(&self) -> bool {
		let ttl = self.ttl.load(Ordering::Relaxed);
		if ttl == 0 {
			false
		} else {
			let now = time::OffsetDateTime::now_utc().unix_timestamp() as u64;
			now >= ttl
		}
	}

	pub fn checked_read(&self) -> Option<&T> {
		if self.is_expired() {
			None
		} else {
			Some(&self.data)
		}
	}

	pub fn update_ttl(&self, ttl: ExMTtlValue) {
		let t: u64 = Self::ttl_value_to_u64(ttl);

		self.ttl.store(t, Ordering::Relaxed);
	}
}

#[derive(Default)]
pub struct ExMemoryPapaya {
	str_map: HashMap<String, WithTtl<String>>,
	i32_map: HashMap<String, WithTtl<AtomicI32>>,
}

impl ExMemoryPapaya {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn remove_expired(&self) {
		self.str_map.pin().retain(|_, v| !v.is_expired());
		self.i32_map.pin().retain(|_, v| !v.is_expired());
	}

	pub fn run_background_cleaner(
		&'static self, interval: Duration, cancellation_token: CancellationToken,
	) -> JoinHandle<()> {
		tokio::spawn(async move {
			loop {
				futures::select! {
					_ = cancellation_token.cancelled().fuse() => {
						break;
					}

					_ = tokio::time::sleep(interval).fuse() => {
						tracing::debug!("running papaya background cleaner. before clean: keys {}", self.str_map.len() + self.i32_map.len());
						self.remove_expired();
						tracing::debug!("running papaya background cleaner. after clean: keys {}", self.str_map.len() + self.i32_map.len());
					}
				}
			}
		})
	}
}

impl ExMemoryBase for ExMemoryPapaya {
	fn get(&self, key: &str) -> impl Future<Output = Result<String, ErrExMemoryGet>> + Send {
		match self.str_map.pin().get(key) {
			Some(v) => match v.checked_read() {
				Some(d) => ready(Ok(d.clone())),
				None => ready(Err(ErrExMemoryGet::NotFound)),
			},
			None => ready(Err(ErrExMemoryGet::NotFound)),
		}
	}

	fn set(
		&self, key: &str, value: String, ttl: ExMTtlValue,
	) -> impl Future<Output = Result<(), ErrExMemorySet>> + Send {
		self.str_map
			.pin()
			.compute(key.to_string(), |entry| match entry {
				Some((_, v)) => {
					if v.is_expired() {
						papaya::Operation::Insert(WithTtl::new(value.clone(), ttl))
					} else {
						papaya::Operation::Abort(())
					}
				},
				None => papaya::Operation::Insert(WithTtl::new(value.clone(), ttl)),
			});
		ready(Ok(()))
	}

	fn upsert(
		&self, key: &str, value: String, ttl: ExMTtlValue,
	) -> impl Future<Output = Result<Option<String>, ErrExMemoryUpsert>> + Send {
		let pin = self.str_map.pin();
		let old = pin.insert(key.to_string(), WithTtl::new(value, ttl));
		let mapped_value = old.map(|v| v.data.clone());
		ready(Ok(mapped_value))
	}

	fn update_ttl(
		&self, key: &str, ttl: ExMTtlValue,
	) -> impl Future<Output = Result<(), ErrExMemoryUpdateTtl>> + Send {
		let pin = self.str_map.pin();
		if let Some(v) = pin.get(key) {
			v.update_ttl(ttl);
		}
		ready(Ok(()))
	}

	fn delete(&self, key: &str) -> impl Future<Output = Result<usize, ErrExMemoryDelete>> + Send {
		match self.str_map.pin().remove(key) {
			Some(_) => ready(Ok(1)),
			None => ready(Ok(0)),
		}
	}

	fn delete_many(
		&self, keys: &[&str],
	) -> impl Future<Output = Result<usize, ErrExMemoryDelete>> + Send {
		let mut sum: usize = 0;
		let pin = self.str_map.pin();
		for key in keys {
			if pin.remove(*key).is_some() {
				sum += 1
			}
		}

		ready(Ok(sum))
	}

	fn get_i32(&self, key: &str) -> impl Future<Output = Result<i32, ErrExMemoryGet>> + Send {
		match self.i32_map.pin().get(key) {
			Some(v) => match v.checked_read() {
				Some(d) => ready(Ok(d.load(Ordering::Relaxed))),
				None => ready(Err(ErrExMemoryGet::NotFound)),
			},
			None => ready(Err(ErrExMemoryGet::NotFound)),
		}
	}

	fn set_i32(
		&self, key: &str, value: i32, ttl: ExMTtlValue,
	) -> impl Future<Output = Result<(), ErrExMemorySet>> + Send {
		self.i32_map
			.pin()
			.compute(key.to_string(), |entry| match entry {
				Some((_, v)) => {
					if v.is_expired() {
						papaya::Operation::Insert(WithTtl::new(AtomicI32::new(value), ttl))
					} else {
						papaya::Operation::Abort(())
					}
				},
				None => papaya::Operation::Insert(WithTtl::new(AtomicI32::new(value), ttl)),
			});
		ready(Ok(()))
	}

	fn upsert_i32(
		&self, key: &str, value: i32, ttl: ExMTtlValue,
	) -> impl Future<Output = Result<Option<i32>, ErrExMemoryUpsert>> + Send {
		let pin = self.i32_map.pin();
		let old = pin.insert(key.to_string(), WithTtl::new(AtomicI32::new(value), ttl));
		let mapped_value = old.map(|v| v.data.load(Ordering::Relaxed));
		ready(Ok(mapped_value))
	}

	fn update_ttl_i32(
		&self, key: &str, ttl: ExMTtlValue,
	) -> impl Future<Output = Result<(), ErrExMemoryUpdateTtl>> + Send {
		let pin = self.i32_map.pin();
		if let Some(v) = pin.get(key) {
			v.update_ttl(ttl);
		}
		ready(Ok(()))
	}

	fn fetch_add_or_set_i32(
		&self, key: &str, value: i32, ttl: Option<ExMTtlValue>,
	) -> impl Future<Output = Result<i32, ErrExMemoryFetchAddOrSet>> + Send {
		let pin = self.i32_map.pin();
		let a = pin.get_or_insert_with(key.to_string(), || {
			WithTtl::new(AtomicI32::new(0), ttl.unwrap_or(ExMTtlValue::NoExp))
		});
		let old = a.data.fetch_add(value, Ordering::Relaxed);

		if let Some(t) = ttl {
			a.update_ttl(t);
		}

		ready(Ok(old))
	}

	fn delete_i32(
		&self, key: &str,
	) -> impl Future<Output = Result<usize, ErrExMemoryDelete>> + Send {
		match self.i32_map.pin().remove(key) {
			Some(_) => ready(Ok(1)),
			None => ready(Ok(0)),
		}
	}

	fn delete_many_i32(
		&self, keys: &[&str],
	) -> impl Future<Output = Result<usize, ErrExMemoryDelete>> + Send {
		let mut sum: usize = 0;

		let pin = self.i32_map.pin();
		for key in keys {
			if pin.remove(*key).is_some() {
				sum += 1
			}
		}

		ready(Ok(sum))
	}
}

impl ExMemory for ExMemoryPapaya {
	fn run_background_workers(
		&'static self, _config: &AppConfig, cancellation_token: CancellationToken,
	) {
		self.run_background_cleaner(std::time::Duration::from_mins(15), cancellation_token);
	}
}
