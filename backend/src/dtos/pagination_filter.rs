use serde::Deserialize;

use crate::constants::MAX_PAGINATION_SIZE;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PaginationFilter {
	page: u32,

	take: u32,
}

impl PaginationFilter {
    pub fn page(&self) -> u32 {
        self.page
    }

    pub fn take(&self) -> u32 {
		if self.take > MAX_PAGINATION_SIZE {
			MAX_PAGINATION_SIZE
		} else {
            self.take
        }
    }
}
