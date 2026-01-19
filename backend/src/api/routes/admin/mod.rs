use crate::{api::state::AxumState, external::repo::ExRepo};
use axum::{Router, routing::get};

pub(crate) mod create;
pub(crate) mod delete;
pub(crate) mod get_list;
pub(crate) mod get_one;
pub(crate) mod update;

pub fn routes_admin<Repo: ExRepo>() -> Router<AxumState<Repo>> {
	Router::new()
		.route(
			"/",
			get(get_list::admin_get_list).post(create::admin_create),
		)
		.route(
			"/{id}",
			get(get_one::admin_get_one)
				.patch(update::admin_update)
				.delete(delete::admin_delete),
		)
}
