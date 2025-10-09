use std::{
	sync::atomic::{AtomicI32, Ordering},
	time::{Duration, Instant},
};

use crate::external::memory::{
	implementations::inmemory::ExternalMemoryInMemory,
	traits::{ExternalMemory, ExternalMemoryPhone},
};

impl ExternalMemoryPhone for ExternalMemoryInMemory {
	async fn phone_otp_get_and_delete_if_exists(
		&self, phone: &str,
	) -> Result<Option<u32>, anyhow::Error> {
		let guard = self.map_phone.guard();
		let value = self.map_phone.get(phone, &guard);
		match value {
			Some(v) => {
				if Instant::now() > v.1 {
					return Ok(None);
				}

				return Ok(Some(v.0));
			},
			None => Ok(None),
		}
	}

	async fn phone_otp_set(&self, phone: &str, code: u32, ttl: u32) -> Result<(), anyhow::Error> {
		let guard = self.map_phone.guard();
		self.map_phone.insert(
			phone.into(),
			(code, Instant::now() + Duration::from_secs(ttl as u64)),
			&guard,
		);
		Ok(())
	}
}
