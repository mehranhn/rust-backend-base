use axum::extract::State;
use utoipa::IntoResponses;

use crate::{
	api::{extractors::Authenticated, responses::RespServerErrorLogged, state::AxumState},
	app::errors::ErrSvAuthLogout,
	external::{memory::ExMemory, repo::ExRepo},
};

#[derive(IntoResponses, custom_macros::AxumResponse)]
pub enum Res {
	#[response(status = 200, description = "Success")]
	Ok,

	#[response(status = 500)]
	ServerError(#[to_response] RespServerErrorLogged),
}

/// Logout
#[utoipa::path(
	post,
	path = "/auth/logout",
	tag = "Auth",
	responses(Res),
	security(
        ("bearerAuth" = [])
    ),
)]
pub async fn auth_logout<D: ExRepo, M: ExMemory>(
	State(s): State<AxumState<D, M>>, auth_data: Authenticated<()>,
) -> Res {
	match s.app.logout(auth_data.session_id).await {
		Ok(_) => Res::Ok,
		Err(e) => match e {
			ErrSvAuthLogout::MemoError(e) => {
				Res::ServerError(RespServerErrorLogged::new(Box::new(e)))
			},
			ErrSvAuthLogout::ServerError(e) => Res::ServerError(e.into()),
		},
	}
}
