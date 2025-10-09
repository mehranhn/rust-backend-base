use std::sync::atomic::{AtomicI32, AtomicU32};

use redis::{aio::{ConnectionManager, ConnectionManagerConfig}, RedisError, RedisResult};

use crate::external::memory::traits::ExternalMemory;

mod phone;

pub struct ExternalMemoryRedis {
	manager: ConnectionManager,
}

impl ExternalMemoryRedis {
	pub async fn new(connection_url: &str) -> Result<Self, RedisError> {
		Ok(Self {
			manager: ConnectionManager::new_with_config(
				redis::Client::open(connection_url)?,
				ConnectionManagerConfig::default(),
			).await?,
		})
	}
}
