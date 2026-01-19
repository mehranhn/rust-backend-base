pub(crate) mod login;
pub(crate) mod login_web;
pub(crate) mod logout;

use crate::{api::state::AxumState, external::repo::ExRepo};
use axum::{Router, routing::post};

pub fn routes_auth<Repo: ExRepo>() -> Router<AxumState<Repo>> {
	Router::new()
		.route("/login", post(login::auth_login))
		.route("/login-web", post(login_web::auth_login_web))
		.route("/logout", post(logout::auth_logout))
}
