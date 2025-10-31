use std::{
	fmt::Display,
	sync::atomic::{AtomicI32, AtomicU32},
	time::{Duration, Instant},
};

use redis::{
	AsyncTypedCommands, RedisError, RedisResult,
	aio::{ConnectionManager, ConnectionManagerConfig},
};
use time::OffsetDateTime;

use crate::external::memory::{
	ExternalMemoryBase,
	errors::{
		ErrExMemoryDelete, ErrExMemoryFetchAddOrSet, ErrExMemoryGet, ErrExMemorySet,
		ErrExMemoryUpdateTtl, ErrExMemoryUpsert,
	},
	types::ExMTtlValue,
};

#[derive(Debug, Clone, Copy)]
enum KeyVariant {
	String,
	I32,
}

impl Display for KeyVariant {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			KeyVariant::String => write!(f, "String"),
			KeyVariant::I32 => write!(f, "I32"),
		}
	}
}

pub struct ExternalMemoryRedis {
	manager: ConnectionManager,
	key_prefix: Option<String>,
}

impl ExternalMemoryRedis {
	pub async fn new(connection_url: &str, key_prefix: Option<String>) -> Result<Self, RedisError> {
		Ok(Self {
			manager: ConnectionManager::new_with_config(
				redis::Client::open(connection_url)?,
				ConnectionManagerConfig::default(),
			)
			.await?,
			key_prefix,
		})
	}

	fn final_key(&self, key: &str, variant: KeyVariant) -> String {
		match self.key_prefix {
			Some(ref prefix) => format!("{prefix}_{variant}_{key}"),
			None => String::from(key),
		}
	}

	async fn _get(&self, key: &str, variant: KeyVariant) -> Result<String, ErrExMemoryGet> {
		let final_key = self.final_key(key, variant);
		let result = self
			.manager
			.clone()
			.get(final_key.as_str())
			.await?
			.ok_or(ErrExMemoryGet::NotFound)?;

		Ok(result)
	}

	async fn _set(
		&self, key: &str, value: String, ttl: ExMTtlValue, variant: KeyVariant,
	) -> Result<(), ErrExMemorySet> {
		let final_key = self.final_key(key, variant);
		let v: String = value.into();

		let mut cmd = redis::cmd("SET");
		cmd.arg(final_key.as_str());
		cmd.arg(v);

		match ttl {
			ExMTtlValue::Duration(d) => {
				cmd.arg("PX");
				cmd.arg(d.whole_milliseconds());
			},
			ExMTtlValue::AtTime(t) => {
				let target_utc = t.assume_utc();

				let now = OffsetDateTime::now_utc();

				let dif = target_utc - now;

				if dif.is_negative() {
					self.manager.clone().del(final_key.as_str()).await?;
					return Ok(());
				}

				cmd.arg("PX");
				cmd.arg(dif.whole_milliseconds());
			},
			ExMTtlValue::NoExp => {},
		};

		let result: Option<String> = cmd.arg("NX").query_async(&mut self.manager.clone()).await?;

		if result.is_none() {
			Err(ErrExMemorySet::KeyExists)
		} else {
			Ok(())
		}
	}

	async fn _upsert(
		&self, key: &str, value: String, ttl: ExMTtlValue, variant: KeyVariant,
	) -> Result<Option<String>, crate::external::memory::errors::ErrExMemoryUpsert> {
		let final_key = self.final_key(key, variant);
		let v: String = value.into();

		let mut cmd = redis::cmd("SET");
		cmd.arg(final_key.as_str());
		cmd.arg(v);

		match ttl {
			ExMTtlValue::Duration(d) => {
				cmd.arg("PX");
				cmd.arg(d.whole_milliseconds());
			},
			ExMTtlValue::AtTime(t) => {
				let target_utc = t.assume_utc();

				let now = OffsetDateTime::now_utc();

				let dif = target_utc - now;

				if dif.is_negative() {
					self.manager.clone().del(final_key.as_str()).await?;
					return Ok(None);
				}

				cmd.arg("PX");
				cmd.arg(dif.whole_milliseconds());
			},
			ExMTtlValue::NoExp => {},
		};

		let result: Option<String> = cmd.query_async(&mut self.manager.clone()).await?;

		Ok(result.map(|v| v.into()))
	}

	async fn _update_ttl(
		&self, key: &str, ttl: ExMTtlValue, variant: KeyVariant,
	) -> Result<(), ErrExMemoryUpdateTtl> {
		let final_key = self.final_key(key, variant);
		match ttl {
			ExMTtlValue::Duration(d) => {
				if self
					.manager
					.clone()
					.pexpire(final_key.as_str(), d.whole_milliseconds() as i64)
					.await?
				{
					Ok(())
				} else {
					Err(ErrExMemoryUpdateTtl::NotFound)
				}
			},
			ExMTtlValue::AtTime(t) => {
				if self
					.manager
					.clone()
					.expire_at(final_key.as_str(), t.assume_utc().unix_timestamp())
					.await?
				{
					Ok(())
				} else {
					Err(ErrExMemoryUpdateTtl::NotFound)
				}
			},
			ExMTtlValue::NoExp => {
				self.manager.clone().persist(key).await?;
				Ok(())
			},
		}
	}

	async fn _delete(&self, key: &str, variant: KeyVariant) -> Result<usize, ErrExMemoryDelete> {
		let final_key = self.final_key(key, variant);

		let deleted: usize = self.manager.clone().del(final_key.as_str()).await?;

		Ok(deleted)
	}

	async fn _delete_many(
		&self, keys: &[&str], variant: KeyVariant,
	) -> Result<usize, ErrExMemoryDelete> {
		let final_keys = keys
			.into_iter()
			.map(|k| self.final_key(k, variant))
			.collect::<Vec<_>>();

		let deleted: usize = self.manager.clone().del(final_keys).await?;

		Ok(deleted)
	}
}

impl ExternalMemoryBase for ExternalMemoryRedis {
	async fn get(&self, key: &str) -> Result<String, ErrExMemoryGet> {
		self._get(key, KeyVariant::String).await
	}

	async fn set(&self, key: &str, value: String, ttl: ExMTtlValue) -> Result<(), ErrExMemorySet> {
		self._set(key, value, ttl, KeyVariant::String).await
	}

	async fn upsert(
		&self, key: &str, value: String, ttl: ExMTtlValue,
	) -> Result<Option<String>, ErrExMemoryUpsert> {
		self._upsert(key, value, ttl, KeyVariant::String).await
	}

	async fn update_ttl(&self, key: &str, ttl: ExMTtlValue) -> Result<(), ErrExMemoryUpdateTtl> {
		self._update_ttl(key, ttl, KeyVariant::String).await
	}

	async fn delete(&self, key: &str) -> Result<usize, ErrExMemoryDelete> {
		self._delete(key, KeyVariant::String).await
	}

	async fn delete_many(&self, keys: &[&str]) -> Result<usize, ErrExMemoryDelete> {
		self._delete_many(keys, KeyVariant::String).await
	}

	async fn get_i32(&self, key: &str) -> Result<i32, ErrExMemoryGet> {
		let v: String = self._get(key, KeyVariant::I32).await?;
		Ok(v.parse::<i32>()?)
	}

	async fn set_i32(&self, key: &str, value: i32, ttl: ExMTtlValue) -> Result<(), ErrExMemorySet> {
		self._set(key, value.to_string(), ttl, KeyVariant::I32)
			.await?;
		Ok(())
	}

	async fn upsert_i32(
		&self, key: &str, value: i32, ttl: ExMTtlValue,
	) -> Result<Option<i32>, ErrExMemoryUpsert> {
		let value = self
			._upsert(key, value.to_string(), ttl, KeyVariant::I32)
			.await?;
		match value {
			Some(v) => Ok(Some(v.parse::<i32>()?)),
			None => Ok(None),
		}
	}

	async fn update_ttl_i32(
		&self, key: &str, ttl: ExMTtlValue,
	) -> Result<(), ErrExMemoryUpdateTtl> {
		self._update_ttl(key, ttl, KeyVariant::I32).await
	}

	async fn fetch_add_or_set_i32(
		&self, key: &str, value: i32, ttl: Option<ExMTtlValue>,
	) -> Result<i32, ErrExMemoryFetchAddOrSet> {
		let final_key = self.final_key(key, KeyVariant::I32);

		let v = self.manager.clone().incr(final_key, value).await?;
		if let Some(t) = ttl {
			match self._update_ttl(key, t, KeyVariant::I32).await {
				Ok(_) => todo!(),
				Err(e) => match e {
					ErrExMemoryUpdateTtl::NotFound => {},
					ErrExMemoryUpdateTtl::ServerError(error) => {
						return Err(ErrExMemoryFetchAddOrSet::ServerError(error));
					},
				},
			}
		}

		Ok(v as i32 - value)
	}

	async fn delete_i32(&self, key: &str) -> Result<usize, ErrExMemoryDelete> {
		let res = self._delete(key, KeyVariant::I32).await?;
		Ok(res)
	}

	async fn delete_many_i32(&self, keys: &[&str]) -> Result<usize, ErrExMemoryDelete> {
		let res = self._delete_many(keys, KeyVariant::I32).await?;
		Ok(res)
	}
}
