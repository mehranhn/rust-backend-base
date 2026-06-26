use axum::{
	body::Body,
	extract::State,
	http::{HeaderValue, header::SET_COOKIE},
	response::{IntoResponse, Response},
};
use utoipa::IntoResponses;

use crate::{
	api::{
		extractors::Authenticated, responses::RespServerErrorLogged, state::AxumState,
		utils::jwt_token_remove_cookie,
	},
	app::errors::ErrSvAuthLogout,
	external::{memory::ExMemory, repo::ExRepo},
};

#[derive(IntoResponses, custom_macros::AxumResponse)]
pub enum Res {
	#[response(status = 200, description = "Success")]
	#[allow(dead_code)]
	Ok,

	#[response(status = 500)]
	ServerError(#[to_response] RespServerErrorLogged),
}

/// Logout
#[utoipa::path(post, path = "/auth/logout-web", tag = "Auth", responses(Res))]
pub async fn auth_logout_web<D: ExRepo, M: ExMemory>(
	State(s): State<AxumState<D, M>>, auth_data: Authenticated<()>,
) -> Response {
	match s.app.logout(auth_data.session_id).await {
		Ok(_) => {
			let mut res = Response::new(Body::empty());
			match HeaderValue::from_str(jwt_token_remove_cookie().as_str()) {
				Ok(hv) => {
					res.headers_mut().insert(SET_COOKIE, hv);
					res
				},
				Err(e) => Res::ServerError(RespServerErrorLogged::new(Box::new(e))).into_response(),
			}
		},
		Err(e) => match e {
			ErrSvAuthLogout::MemoError(e) => {
				Res::ServerError(RespServerErrorLogged::new(Box::new(e))).into_response()
			},
			ErrSvAuthLogout::ServerError(e) => Res::ServerError(e.into()).into_response(),
		},
	}
}
