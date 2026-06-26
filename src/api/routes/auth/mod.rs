pub(crate) mod login;
pub(crate) mod login_web;
pub(crate) mod logout;
pub(crate) mod logout_web;

use crate::{
	api::state::AxumState,
	external::{memory::ExMemory, repo::ExRepo},
};
use axum::{Router, routing::post};

pub fn routes_auth<D: ExRepo, M: ExMemory>() -> Router<AxumState<D, M>> {
	Router::new()
		.route("/login", post(login::auth_login))
		.route("/login-web", post(login_web::auth_login_web))
		.route("/logout", post(logout::auth_logout))
		.route("/logout-web", post(logout_web::auth_logout_web))
}
