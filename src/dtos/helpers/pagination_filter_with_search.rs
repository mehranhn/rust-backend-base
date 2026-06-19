use serde::Deserialize;
use utoipa::{
	IntoParams, PartialSchema,
	openapi::path::{Parameter, ParameterBuilder},
};

use crate::dtos::PaginationFilter;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
pub struct PaginationFilterWithSearch {
	#[serde(flatten)]
	page_take: PaginationFilter,

	#[serde(default)]
	search: String,
}

impl PaginationFilterWithSearch {
	pub fn page(&self) -> u64 {
		self.page_take.page()
	}

	pub fn skip(&self) -> u64 {
		self.page_take.skip()
	}

	pub fn take(&self) -> u64 {
		self.page_take.take()
	}

	pub fn search(&self) -> &String {
		&self.search
	}
}

impl IntoParams for PaginationFilterWithSearch {
	fn into_params(
		parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
	) -> Vec<utoipa::openapi::path::Parameter> {
		#![allow(clippy::vec_init_then_push)]
		let mut res = Vec::<Parameter>::with_capacity(3);

		res.push(
			ParameterBuilder::new()
				.name("page")
				.parameter_in(parameter_in_provider().unwrap_or_default())
				.description(Some("Page"))
				.schema(Some(u64::schema()))
				.required(utoipa::openapi::Required::False)
				.build(),
		);

		res.push(
			ParameterBuilder::new()
				.name("take")
				.parameter_in(parameter_in_provider().unwrap_or_default())
				.description(Some("Take"))
				.schema(Some(u64::schema()))
				.required(utoipa::openapi::Required::False)
				.build(),
		);

		res.push(
			ParameterBuilder::new()
				.name("search")
				.parameter_in(parameter_in_provider().unwrap_or_default())
				.description(Some("Search"))
				.schema(Some(String::schema()))
				.required(utoipa::openapi::Required::False)
				.build(),
		);

		res
	}
}
