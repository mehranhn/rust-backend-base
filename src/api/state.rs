use crate::{
	app::App,
	external::{memory::ExMemory, repo::ExRepo},
};

pub struct AxumState<D: ExRepo, M: ExMemory> {
	pub app: &'static App<D, M>,
}

impl<D: ExRepo, M: ExMemory> AxumState<D, M> {
	pub fn new(app: &'static App<D, M>) -> Self {
		Self { app }
	}
}

impl<D: ExRepo, M: ExMemory> Clone for AxumState<D, M> {
	fn clone(&self) -> Self {
		Self { app: self.app }
	}
}
