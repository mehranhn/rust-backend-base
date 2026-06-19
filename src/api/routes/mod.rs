pub(crate) mod admin;
pub(crate) mod auth;

use axum::{Router, body::Body, http::StatusCode, response::Response};

use crate::{api::state::AxumState, external::repo::ExRepo};

async fn fallback() -> Response {
	let mut r = Response::new(Body::from("Route Not Found"));
	*r.status_mut() = StatusCode::NOT_FOUND;
	r
}

pub fn routes_api<Repo: ExRepo>() -> Router<AxumState<Repo>> {
	Router::new()
		.nest("/user/admin", admin::routes_admin())
		.nest("/auth", auth::routes_auth())
		.fallback(fallback)
}
