use std::marker::Send;

use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use crate::{
	app::errors::ErrServerError, dtos::{
		AdminCreateDto, AdminDto, AdminDtoSortColumns, AdminUpdateDto, NullUndefinedValue,
		PaginatedResult, PaginationFilterWithSearchOrder,
	}, external::repo::{
		ExRepoAdmin,
		errors::{
			ErrExRepoAdminCreate, ErrExRepoAdminDelete, ErrExRepoAdminGetById, ErrExRepoAdminUpdate,
		},
		implementations::sea_orm_postgres::{
			models,
			types::roles::Roles,
			utils::{DbHandle, DbHandleInner},
		},
	}, utils::generate_uuid
};

impl<T: DbHandleInner + Send> ExRepoAdmin for DbHandle<T> {
	async fn admin_get_list(
		&mut self, filter: PaginationFilterWithSearchOrder<AdminDtoSortColumns>,
	) -> Result<PaginatedResult<AdminDto>, ErrServerError> {
		let mut query = models::user::Entity::find()
			.filter(models::user::COLUMN.role.eq(Roles::Admin as i32))
			.filter(models::user::COLUMN.username.contains(filter.search()));

		let ord = if filter.order_by_is_asc() {
			sea_orm::Order::Asc
		} else {
			sea_orm::Order::Desc
		};

		if let Some(c) = filter.order_by_column() {
			query = match c {
				AdminDtoSortColumns::Id => query.order_by(models::user::Column::Id, ord),
				AdminDtoSortColumns::CreatedAt => {
					query.order_by(models::user::Column::CreatedAt, ord)
				},
				AdminDtoSortColumns::Username => {
					query.order_by(models::user::Column::Username, ord)
				},
				AdminDtoSortColumns::Phone => query.order_by(models::user::Column::Phone, ord),
				AdminDtoSortColumns::Email => query.order_by(models::user::Column::Email, ord),
			}
		};

		let cur = query.paginate(self, filter.take());

		let data = cur.fetch_page(filter.page_index()).await?;
		let count = cur.num_items().await?;

		Ok(PaginatedResult::new(
			data.into_iter().map(|d| d.into()).collect(),
			count,
		))
	}

	async fn admin_get_by_id(&mut self, id: Uuid) -> Result<AdminDto, ErrExRepoAdminGetById> {
		let Some(admin) = models::user::Entity::find_by_id(id)
			.filter(models::user::COLUMN.role.eq(Roles::Admin as i32))
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
		let active_model = models::user::ActiveModel {
			id: ActiveValue::Set(generate_uuid()),
			role: ActiveValue::Set(Roles::Admin),
			username: ActiveValue::Set(dto.username.into_inner()),
			hashed_password: ActiveValue::Set(dto.password),
			phone: ActiveValue::Set(dto.phone.map(|p| p.into_inner())),
			email: ActiveValue::Set(dto.email.map(|e| e.into_inner())),
			..Default::default()
		};

		models::user::Entity::insert(active_model)
			.exec(self)
			.await?;

		Ok(())
	}

	async fn admin_update(
		&mut self, id: Uuid, dto: AdminUpdateDto,
	) -> Result<(), ErrExRepoAdminUpdate> {
		let mut active_model = models::user::ActiveModel {
			..Default::default()
		};

		active_model.username = match dto.username {
			Some(u) => ActiveValue::Set(u),
			_ => ActiveValue::NotSet,
		};

		active_model.phone = match dto.phone {
			NullUndefinedValue::Some(p) => ActiveValue::Set(Some(p.into_inner())),
			NullUndefinedValue::Null => ActiveValue::Set(None),
			NullUndefinedValue::Undefined => ActiveValue::NotSet,
		};

		active_model.email = match dto.email {
			NullUndefinedValue::Some(e) => ActiveValue::Set(Some(e.into_inner())),
			NullUndefinedValue::Null => ActiveValue::Set(None),
			NullUndefinedValue::Undefined => ActiveValue::NotSet,
		};

		let res = models::user::Entity::update_many()
			.set(active_model)
			.filter(models::user::COLUMN.id.eq(id))
			.exec(self)
			.await?;

		if res.rows_affected == 0 {
			return Err(ErrExRepoAdminUpdate::NotFound);
		}

		Ok(())
	}

	async fn admin_delete(&mut self, id: Uuid) -> Result<(), ErrExRepoAdminDelete> {
		let res = models::user::Entity::delete_by_id(id).exec(self).await?;

		if res.rows_affected == 0 {
			return Err(ErrExRepoAdminDelete::NotFound);
		}

		Ok(())
	}
}
