use axum::{
	Json,
	extract::{Path, State},
};
use utoipa::IntoResponses;
use uuid::Uuid;

use crate::{
	api::{extractors::Authenticated, responses::RespServerErrorLogged, state::AxumState},
	dtos::AdminUpdateDto,
	external::repo::{ExRepo, errors::ErrExRepoAdminUpdate},
	permission::PermAdminUpdate,
	app::errors::ErrSvAdminUpdate,
};

#[derive(IntoResponses, custom_macros::AxumResponse)]
pub enum Res {
	#[response(status = 200, description = "Success")]
	Ok,

	#[response(status = 404, description = "Not Found")]
	NotFound,

	#[response(status = 500)]
	ServerError(#[to_response] RespServerErrorLogged),
}

/// Update an admin
#[utoipa::path(
	patch,
	path = "/user/admin/{id}",
	tag = "Admin",
	responses(Res),
	security(
        ("bearerAuth" = ["AdminUpdate"])
    ),
)]
pub async fn admin_update<Repo: ExRepo>(
	State(s): State<AxumState<Repo>>, _: Authenticated<PermAdminUpdate>, Path(id): Path<Uuid>,
	Json(data): Json<AdminUpdateDto>,
) -> Res {
	match s.app.admin_update(id, data).await {
		Ok(_) => Res::Ok,
		Err(e) => match e {
			ErrSvAdminUpdate::RepoError(e) => match e {
				ErrExRepoAdminUpdate::NotFound => Res::NotFound,
				ErrExRepoAdminUpdate::ServerError(error) => Res::ServerError(error.into()),
			},
			ErrSvAdminUpdate::ServerError(error) => Res::ServerError(error.into()),
		},
	}
}
