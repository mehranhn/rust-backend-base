use axum::{
	Json,
	body::Body,
	extract::State,
	http::{HeaderValue, header::SET_COOKIE},
	response::{IntoResponse, Response},
};
use utoipa::IntoResponses;

use crate::{
	api::{responses::RespServerErrorLogged, state::AxumState, utils::jwt_token_into_cookie},
	app::errors::ErrSvAuthLogin,
	dtos::LoginDto,
	external::{
		memory::ExMemory,
		repo::{ExRepo, errors::ErrExRepoUserGetUserByUsername},
	},
	utils::encode_to_jwt,
};

#[derive(IntoResponses, custom_macros::AxumResponse)]
pub enum Res {
	#[response(status = 400, description = "User Not Found")]
	UserNotFound,

	#[response(status = 500)]
	ServerError(#[to_response] RespServerErrorLogged),
}

/// Login With Cookie
#[utoipa::path(post, path = "/auth/login-web", tag = "Auth", responses(Res))]
pub async fn auth_login_web<D: ExRepo, M: ExMemory>(
	State(s): State<AxumState<D, M>>, Json(data): Json<LoginDto>,
) -> Response {
	match s.app.login(data).await {
		Ok(auth_data) => {
			let expire_at = auth_data.expire_at;
			match encode_to_jwt(
				auth_data,
				&s.app.config().jwt_secret,
				s.app.config().jwt_exp_after,
			) {
				Ok(token) => {
					let mut res = Response::new(Body::empty());
					match HeaderValue::from_str(
						jwt_token_into_cookie(token.as_str(), expire_at).as_str(),
					) {
						Ok(hv) => {
							res.headers_mut().insert(SET_COOKIE, hv);
							res
						},
						Err(e) => Res::ServerError(RespServerErrorLogged::new(Box::new(e)))
							.into_response(),
					}
				},
				Err(e) => Res::ServerError(RespServerErrorLogged::new(Box::new(e))).into_response(),
			}
		},
		Err(e) => match e {
			ErrSvAuthLogin::IncorrectPassword => Res::UserNotFound.into_response(),
			ErrSvAuthLogin::RepoError(e) => match e {
				ErrExRepoUserGetUserByUsername::NotFound => Res::UserNotFound.into_response(),
				ErrExRepoUserGetUserByUsername::ServerError(error) => {
					Res::ServerError(error.into()).into_response()
				},
			},
			ErrSvAuthLogin::ServerError(error) => Res::ServerError(error.into()).into_response(),
		},
	}
}
