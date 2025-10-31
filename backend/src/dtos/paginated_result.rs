use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PaginatedResult<T> {
	data: Vec<T>,
	count: u64,
}
