use uuid::Uuid;

use crate::{
	dtos::{PaginatedResult, PaginationFilterWithSearchOrder},
	external::repo::ExRepoBase,
	modules::user::{
		dtos::{UserCreateDto, UserDto, UserUpdateDto},
		enums::UserSortColumns,
	},
};

pub trait ExRepoUser: ExRepoBase {
	fn user_get_list(
		&self, tx: &mut Self::Connection, filter: PaginationFilterWithSearchOrder<UserSortColumns>,
	) -> impl Future<Output = Result<PaginatedResult<UserDto>, ()>> + Send;
	async fn user_get_by_id(
		&self, tx: &mut Self::Connection, id: Uuid,
	) -> impl Future<Output = Result<UserDto, ()>> + Send;
	async fn user_create(
		&self, tx: &mut Self::Connection, dto: UserCreateDto,
	) -> impl Future<Output = Result<(), ()>> + Send;
	async fn user_upsert(
		&self, tx: &mut Self::Connection, id: Uuid, dto: UserCreateDto,
	) -> impl Future<Output = Result<(), ()>> + Send;
	async fn user_update(
		&self, tx: &mut Self::Connection, id: Uuid, dto: UserUpdateDto,
	) -> impl Future<Output = Result<(), ()>> + Send;
	async fn user_delete(
		&self, tx: &mut Self::Connection, id: Uuid,
	) -> impl Future<Output = Result<(), ()>> + Send;
}
