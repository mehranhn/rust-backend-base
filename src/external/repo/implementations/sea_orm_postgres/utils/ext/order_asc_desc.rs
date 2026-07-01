use utoipa::PartialSchema;

use crate::dtos::PaginationFilterWithSearchOrder;

impl<T: Copy + PartialSchema> Into<sea_orm::Order> for &PaginationFilterWithSearchOrder<T> {
	fn into(self) -> sea_orm::Order {
		if self.order_by_is_asc() {
			sea_orm::Order::Asc
		} else {
			sea_orm::Order::Desc
		}
	}
}
