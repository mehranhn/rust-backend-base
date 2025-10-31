use serde::Deserialize;

use crate::{
	constants::MAX_PAGINATION_SIZE,
	dtos::{FilterOrderBy, PaginationFilterWithSearch},
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PaginationFilterWithSearchOrder<T> {
    #[serde(flatten)]
	page_take_search: PaginationFilterWithSearch,

	order_by: Vec<FilterOrderBy<T>>,
}

impl<T> PaginationFilterWithSearchOrder<T> {
	pub fn page(&self) -> u32 {
		self.page_take_search.page()
	}

	pub fn take(&self) -> u32 {
		self.page_take_search.take()
	}

	pub fn search(&self) -> &String {
		self.page_take_search.search()
	}

    pub fn order_by(&self) -> &Vec<FilterOrderBy<T>> {
        &self.order_by
    }
}
