use axum::extract::{Query, State};
use utoipa::IntoResponses;

use crate::{
	api::{extractors::Authenticated, responses::RespServerErrorLogged, state::AxumState},
	dtos::{AdminDto, AdminDtoSortColumns, PaginatedResult, PaginationFilterWithSearchOrder},
	external::{memory::ExMemory, repo::ExRepo},
	permission::PermAdminRead,
};

#[derive(IntoResponses, custom_macros::AxumResponse)]
pub enum Res {
	#[response(status = 200, description = "Success")]
	Ok(#[json] PaginatedResult<AdminDto>),

	#[response(status = 500)]
	ServerError(#[to_response] RespServerErrorLogged),
}

/// Get admin list
#[utoipa::path(
	get,
	path = "/user/admin",
	tag = "Admin",
	responses(Res),
	params(PaginationFilterWithSearchOrder<AdminDtoSortColumns>),
	security(
        ("bearerAuth" = ["AdminRead"])
    ),
)]
pub async fn admin_get_list<D: ExRepo, M: ExMemory>(
	State(s): State<AxumState<D, M>>, _: Authenticated<PermAdminRead>,
	Query(filter): Query<PaginationFilterWithSearchOrder<AdminDtoSortColumns>>,
) -> Res {
	match s.app.admin_get_list(filter).await {
		Ok(d) => Res::Ok(d),
		Err(e) => Res::ServerError(e.into()),
	}
}
