use axum::extract::{Path, State};
use utoipa::IntoResponses;
use uuid::Uuid;

use crate::{
	api::{extractors::Authenticated, responses::RespServerErrorLogged, state::AxumState},
	external::repo::{ExRepo, errors::ErrExRepoAdminDelete},
	permission::PermAdminDelete,
	app::errors::ErrSvAdminDelete,
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

/// Delete an admin
#[utoipa::path(
	delete,
	path = "/user/admin/{id}",
	tag = "Admin",
	responses(Res),
	security(
        ("bearerAuth" = ["AdminDelete"])
    ),
)]
pub async fn admin_delete<Repo: ExRepo>(
	State(s): State<AxumState<Repo>>, _: Authenticated<PermAdminDelete>, Path(id): Path<Uuid>,
) -> Res {
	match s.app.admin_delete(id).await {
		Ok(_) => Res::Ok,
		Err(e) => match e {
			ErrSvAdminDelete::RepoError(e) => match e {
				ErrExRepoAdminDelete::NotFound => Res::NotFound,
				ErrExRepoAdminDelete::ServerError(error) => Res::ServerError(error.into()),
			},
			ErrSvAdminDelete::ServerError(error) => Res::ServerError(error.into()),
		},
	}
}
