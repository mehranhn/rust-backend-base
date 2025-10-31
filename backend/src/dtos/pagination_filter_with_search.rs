use serde::Deserialize;

use crate::{constants::MAX_PAGINATION_SIZE, dtos::PaginationFilter};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PaginationFilterWithSearch {
    #[serde(flatten)]
    page_take: PaginationFilter,

    search: String,
}

impl PaginationFilterWithSearch {
    pub fn page(&self) -> u32 {
        self.page_take.page()
    }

    pub fn take(&self) -> u32 {
        self.page_take.take()
    }

    pub fn search(&self) -> &String {
        &self.search
    }
}
