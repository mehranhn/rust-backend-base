use axum::{Json, extract::State};
use utoipa::IntoResponses;

use crate::{
	api::{extractors::Authenticated, responses::RespServerErrorLogged, state::AxumState},
	app::errors::ErrSvAdminCreate,
	dtos::AdminCreateDto,
	external::{
		memory::ExMemory,
		repo::{ExRepo, errors::ErrExRepoAdminCreate},
	},
	permission::PermAdminCreate,
	validators::StringVPassword,
};

#[derive(IntoResponses, custom_macros::AxumResponse)]
pub enum Res {
	#[response(status = 200, description = "Success")]
	Ok,

	#[response(status = 409, description = "Unique Violation")]
	UniqueViolation,

	#[response(status = 500)]
	ServerError(#[to_response] RespServerErrorLogged),
}

/// Create an admin
#[utoipa::path(
	post,
	path = "/user/admin",
	tag = "Admin",
	responses(Res),
	security(
        ("bearerAuth" = ["AdminCreate"])
    ),
)]
pub async fn admin_create<D: ExRepo, M: ExMemory>(
	State(s): State<AxumState<D, M>>, _: Authenticated<PermAdminCreate>,
	Json(data): Json<AdminCreateDto<StringVPassword>>,
) -> Res {
	match s.app.admin_create(data).await {
		Ok(_) => Res::Ok,
		Err(e) => match e {
			ErrSvAdminCreate::RepoError(e) => match e {
				ErrExRepoAdminCreate::UniqueUserName => Res::UniqueViolation,
				ErrExRepoAdminCreate::ServerError(error) => Res::ServerError(error.into()),
			},
			ErrSvAdminCreate::ServerError(error) => Res::ServerError(error.into()),
		},
	}
}
