use serde::{Deserialize, Deserializer};
use utoipa::{
	IntoParams, PartialSchema,
	openapi::path::{Parameter, ParameterBuilder},
};

use crate::constants::MAX_PAGINATION_SIZE;

fn de_u64_from_str<'de, D>(d: D) -> Result<u64, D::Error>
where
	D: Deserializer<'de>,
{
	let s = String::deserialize(d)?;
	s.parse::<u64>().map_err(serde::de::Error::custom)
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PaginationFilter {
	#[serde(default = "PaginationFilter::default_page")]
	#[serde(deserialize_with = "de_u64_from_str")]
	page: u64,

	#[serde(default = "PaginationFilter::default_take")]
	#[serde(deserialize_with = "de_u64_from_str")]
	take: u64,
}

impl Default for PaginationFilter {
	fn default() -> Self {
		Self {
			page: Self::default_page(),
			take: Self::default_take(),
		}
	}
}

impl PaginationFilter {
	pub fn page(&self) -> u64 {
		self.page
	}

	pub fn page_index(&self) -> u64 {
		self.page() - 1
	}

	pub fn skip(&self) -> u64 {
		self.page_index() * self.take()
	}

	pub fn take(&self) -> u64 {
		if self.take > MAX_PAGINATION_SIZE {
			MAX_PAGINATION_SIZE
		} else {
			self.take
		}
	}

	fn default_page() -> u64 {
		1
	}

	fn default_take() -> u64 {
		10
	}
}

impl IntoParams for PaginationFilter {
	fn into_params(
		parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
	) -> Vec<utoipa::openapi::path::Parameter> {
		#![allow(clippy::vec_init_then_push)]
		let mut res = Vec::<Parameter>::with_capacity(2);

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

		res
	}
}
