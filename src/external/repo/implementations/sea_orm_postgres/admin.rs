use std::marker::Send;

use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use crate::{
	app::errors::ErrServerError,
	dtos::{
		AdminCreateDto, AdminDto, AdminDtoSortColumns, AdminUpdateDto, PaginatedResult,
		PaginationFilterWithSearchOrder,
	},
	external::repo::{
		ExRepoAdmin,
		errors::{
			ErrExRepoAdminCreate, ErrExRepoAdminDelete, ErrExRepoAdminGetById, ErrExRepoAdminUpdate,
		},
		implementations::sea_orm_postgres::{
			models::*,
			types::roles::Roles,
			utils::{DbHandle, DbHandleInner},
		},
	},
};

impl<T: DbHandleInner + Send> ExRepoAdmin for DbHandle<T> {
	async fn admin_get_list(
		&mut self, filter: PaginationFilterWithSearchOrder<AdminDtoSortColumns>,
	) -> Result<PaginatedResult<AdminDto>, ErrServerError> {
		let mut query = user::Entity::find()
			.filter(user::COLUMN.role.eq(Roles::Admin as i32))
			.filter(user::COLUMN.username.contains(filter.search()));

		if let Some(c) = filter.order_by_column() {
			query = query.order_by(user::Column::from(c), (&filter).into());
		}

		let cur = query.paginate(self, filter.take());

		let data = cur.fetch_page(filter.page_index()).await?;
		let count = cur.num_items().await?;

		Ok(PaginatedResult::new(
			data.into_iter().map(|d| d.into()).collect(),
			count,
		))
	}

	async fn admin_get_by_id(&mut self, id: Uuid) -> Result<AdminDto, ErrExRepoAdminGetById> {
		let Some(admin) = user::Entity::find_by_id(id)
			.filter(user::COLUMN.role.eq(Roles::Admin as i32))
			.one(self)
			.await?
		else {
			return Err(ErrExRepoAdminGetById::NotFound);
		};

		Ok(admin.into())
	}

	async fn admin_create(
		&mut self, dto: AdminCreateDto<Vec<u8>>,
	) -> Result<(), ErrExRepoAdminCreate> {
		user::Entity::insert(dto.into_active_model())
			.exec(self)
			.await?;

		Ok(())
	}

	async fn admin_update(
		&mut self, id: Uuid, dto: AdminUpdateDto,
	) -> Result<(), ErrExRepoAdminUpdate> {
		let res = user::Entity::update_many()
			.set(dto.into_active_model())
			.filter(user::COLUMN.id.eq(id))
			.exec(self)
			.await?;

		if res.rows_affected == 0 {
			return Err(ErrExRepoAdminUpdate::NotFound);
		}

		Ok(())
	}

	async fn admin_delete(&mut self, id: Uuid) -> Result<(), ErrExRepoAdminDelete> {
		let res = user::Entity::delete_by_id(id).exec(self).await?;

		if res.rows_affected == 0 {
			return Err(ErrExRepoAdminDelete::NotFound);
		}

		Ok(())
	}
}
