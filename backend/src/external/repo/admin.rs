use uuid::Uuid;

use crate::{
	dtos::{
		AdminCreateDto, AdminDto, AdminDtoSortColumns, AdminUpdateDto, PaginatedResult,
		PaginationFilterWithSearchOrder,
	},
	external::repo::errors::{
		ErrExRepoAdminCreate, ErrExRepoAdminDelete, ErrExRepoAdminGetById, ErrExRepoAdminUpdate,
	},
	app::errors::ErrServerError,
};

pub trait ExRepoAdmin: Send {
	fn admin_get_list(
		&mut self, filter: PaginationFilterWithSearchOrder<AdminDtoSortColumns>,
	) -> impl Future<Output = Result<PaginatedResult<AdminDto>, ErrServerError>> + Send;

	fn admin_get_by_id(
		&mut self, id: Uuid,
	) -> impl Future<Output = Result<AdminDto, ErrExRepoAdminGetById>> + Send;

	fn admin_create(
		&mut self, id: Uuid, dto: AdminCreateDto<Vec<u8>>,
	) -> impl Future<Output = Result<(), ErrExRepoAdminCreate>> + Send;

	fn admin_update(
		&mut self, id: Uuid, dto: AdminUpdateDto,
	) -> impl Future<Output = Result<(), ErrExRepoAdminUpdate>> + Send;

	fn admin_delete(
		&mut self, id: Uuid,
	) -> impl Future<Output = Result<(), ErrExRepoAdminDelete>> + Send;
}
