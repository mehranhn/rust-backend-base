use axum::extract::State;
use utoipa::IntoResponses;

use crate::{
	api::{extractors::Authenticated, responses::RespServerErrorLogged, state::AxumState},
	external::repo::ExRepo,
};

#[derive(IntoResponses, custom_macros::AxumResponse)]
pub enum Res {
	#[response(status = 200, description = "Success")]
	Ok,

	#[response(status = 500)]
	ServerError(#[to_response] RespServerErrorLogged),
}

/// Logout
#[utoipa::path(post, path = "/auth/logout", tag = "Auth", responses(Res))]
pub async fn auth_logout<Repo: ExRepo>(
	State(s): State<AxumState<Repo>>, auth_data: Authenticated<()>,
) -> Res {
	match s.app.logout(auth_data.session_id).await {
		Ok(_) => Res::Ok,
		Err(e) => Res::ServerError(e.into()),
	}
}
