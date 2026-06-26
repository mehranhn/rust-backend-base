use axum::extract::{Path, State};
use utoipa::IntoResponses;
use uuid::Uuid;

use crate::{
	api::{extractors::Authenticated, responses::RespServerErrorLogged, state::AxumState},
	app::errors::ErrSvAdminGetById,
	dtos::AdminDto,
	external::{
		memory::ExMemory,
		repo::{ExRepo, errors::ErrExRepoAdminGetById},
	},
	permission::PermAdminRead,
};

#[derive(IntoResponses, custom_macros::AxumResponse)]
pub enum Res {
	#[response(status = 200, description = "Success")]
	Ok(#[json] AdminDto),

	#[response(status = 404, description = "Not Found")]
	NotFound,

	#[response(status = 500)]
	ServerError(#[to_response] RespServerErrorLogged),
}

/// Get an admin
#[utoipa::path(
	get,
	path = "/user/admin/{id}",
	tag = "Admin",
	responses(Res),
	security(
        ("bearerAuth" = ["AdminRead"])
    ),
)]
pub async fn admin_get_one<D: ExRepo, M: ExMemory>(
	State(s): State<AxumState<D, M>>, _: Authenticated<PermAdminRead>, Path(id): Path<Uuid>,
) -> Res {
	match s.app.admin_get_by_id(id).await {
		Ok(d) => Res::Ok(d),
		Err(e) => match e {
			ErrSvAdminGetById::RepoError(e) => match e {
				ErrExRepoAdminGetById::NotFound => Res::NotFound,
				ErrExRepoAdminGetById::ServerError(error) => Res::ServerError(error.into()),
			},
			ErrSvAdminGetById::ServerError(error) => Res::ServerError(error.into()),
		},
	}
}
