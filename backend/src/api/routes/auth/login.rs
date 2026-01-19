use axum::{Json, extract::State};
use utoipa::IntoResponses;

use crate::{
	api::{responses::RespServerErrorLogged, state::AxumState},
	app::errors::ErrSvAuthLogin,
	dtos::LoginDto,
	external::repo::{ExRepo, errors::ErrExRepoUserGetUserByUsername},
	utils::encode_to_jwt,
};

#[derive(IntoResponses, custom_macros::AxumResponse)]
pub enum Res {
	#[response(status = 200, description = "Success")]
	Ok(String),

	#[response(status = 400, description = "User Not Found")]
	UserNotFound,

	#[response(status = 500)]
	ServerError(#[to_response] RespServerErrorLogged),
}

/// Login With Bearer Token
#[utoipa::path(
	post,
	path = "/auth/login",
	tag = "Auth",
	responses(
		(status = 200, description = "Success"),
		Res,
	)
)]
pub async fn auth_login<Repo: ExRepo>(
	State(s): State<AxumState<Repo>>, Json(data): Json<LoginDto>,
) -> Res {
	match s.app.login(data).await {
		Ok(auth_data) => match encode_to_jwt(
			auth_data,
			&s.app.config().jwt_secret,
			s.app.config().jwt_exp_after,
		) {
			Ok(token) => Res::Ok(token),
			Err(e) => Res::ServerError(RespServerErrorLogged::new(Box::new(e))),
		},
		Err(e) => match e {
			ErrSvAuthLogin::IncorrectPassword => Res::UserNotFound,
			ErrSvAuthLogin::RepoError(e) => match e {
				ErrExRepoUserGetUserByUsername::NotFound => Res::UserNotFound,
				ErrExRepoUserGetUserByUsername::ServerError(error) => {
					Res::ServerError(error.into())
				},
			},
			ErrSvAuthLogin::ServerError(error) => Res::ServerError(error.into()),
		},
	}
}
