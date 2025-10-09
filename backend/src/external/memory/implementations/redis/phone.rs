use std::sync::atomic::{AtomicI32, Ordering};

use redis::AsyncCommands;

use crate::external::memory::{
	implementations::{inmemory::ExternalMemoryInMemory, redis::ExternalMemoryRedis},
	traits::{ExternalMemory, ExternalMemoryPhone},
};

impl ExternalMemoryPhone for ExternalMemoryRedis {
	async fn phone_otp_get_and_delete_if_exists(
		&self, phone: &str,
	) -> Result<Option<u32>, anyhow::Error> {
		let mut connection = self.manager.clone();
		Ok(connection.get_del::<_, Option<u32>>(format!("phone_{phone}")).await?)
	}

	async fn phone_otp_set(
		&self, phone: &str, code: u32, seconds: u32
	) -> Result<(), anyhow::Error> {
		let mut connection = self.manager.clone();
        connection.set_ex::<_, _, ()>(phone, code, seconds as u64).await?;
        Ok(())
	}
}
