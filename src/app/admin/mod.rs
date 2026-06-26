use uuid::Uuid;

use crate::{
	app::{
		App,
		admin::errors::{ErrSvAdminCreate, ErrSvAdminDelete, ErrSvAdminGetById, ErrSvAdminUpdate},
		errors::ErrServerError,
	},
	dtos::{
		AdminCreateDto, AdminDto, AdminDtoSortColumns, AdminUpdateDto, PaginatedResult,
		PaginationFilterWithSearchOrder,
	},
	external::{
		memory::ExMemory,
		repo::{ExRepo, ExRepoAdmin},
	},
	validators::StringVPassword,
};

pub mod errors;
mod utils;

impl<D: ExRepo, M: ExMemory> App<D, M> {
	pub async fn admin_get_list(
		&self, filter: PaginationFilterWithSearchOrder<AdminDtoSortColumns>,
	) -> Result<PaginatedResult<AdminDto>, ErrServerError> {
		let mut c = self.repo.connection().await?;
		let list = c.admin_get_list(filter).await?;

		Ok(list)
	}

	pub async fn admin_get_by_id(&self, id: Uuid) -> Result<AdminDto, ErrSvAdminGetById> {
		let mut c = self.repo.connection().await?;
		let admin = c.admin_get_by_id(id).await?;

		Ok(admin)
	}

	pub async fn admin_create(
		&self, dto: AdminCreateDto<StringVPassword>,
	) -> Result<(), ErrSvAdminCreate> {
		let mut c = self.repo.connection().await?;
		c.admin_create(dto.into_hashed()).await?;

		Ok(())
	}

	pub async fn admin_update(
		&self, id: Uuid, dto: AdminUpdateDto,
	) -> Result<(), ErrSvAdminUpdate> {
		let mut c = self.repo.connection().await?;
		c.admin_update(id, dto).await?;

		Ok(())
	}

	pub async fn admin_delete(&self, id: Uuid) -> Result<(), ErrSvAdminDelete> {
		let mut c = self.repo.connection().await?;
		c.admin_delete(id).await?;

		Ok(())
	}
}
