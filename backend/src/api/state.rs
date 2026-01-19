use crate::{external::repo::ExRepo, app::App};

pub struct AxumState<Repo: ExRepo> {
	pub app: &'static App<Repo>,
}

impl<Repo: ExRepo> AxumState<Repo> {
	pub fn new(app: &'static App<Repo>) -> Self {
		Self { app }
	}
}

impl<Repo: ExRepo> Clone for AxumState<Repo> {
	fn clone(&self) -> Self {
		Self {
			app: self.app,
		}
	}
}
