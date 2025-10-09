use std::{
	sync::{
		Arc,
		atomic::{AtomicI32, AtomicU32},
	},
	time::{Duration, Instant},
};

use tokio::{select, task::AbortHandle};
use tokio_util::sync::CancellationToken;

use crate::external::memory::traits::ExternalMemory;

mod phone;

pub struct ExternalMemoryInMemory {
	map_phone: Arc<flurry::HashMap<String, (u32, Instant)>>,
	abort_handle: AbortHandle,
}

impl ExternalMemoryInMemory {
	pub fn new() -> Self {
		let map_phone: Arc<flurry::HashMap<String, (u32, Instant)>> = Arc::new(Default::default());

		let map_phone2 = map_phone.clone();
		let abort_handle = tokio::spawn(async move {
			let mut interval = tokio::time::interval(Duration::from_secs(60));
			interval.tick().await;

			loop {
				interval.tick().await;
				let now = Instant::now();
				map_phone2.pin().retain(|_, v| v.1 < now);
			}
		})
		.abort_handle();

		Self {
			map_phone,
			abort_handle,
		}
	}
}

impl Drop for ExternalMemoryInMemory {
	fn drop(&mut self) {
		self.abort_handle.abort();
	}
}

impl ExternalMemory for ExternalMemoryInMemory {}
