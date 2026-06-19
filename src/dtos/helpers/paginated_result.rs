use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct PaginatedResult<T> {
	data: Vec<T>,
	count: u64,
}

impl<T> PaginatedResult<T> {
	pub fn new(data: Vec<T>, count: u64) -> Self {
		Self { data, count }
	}
}
