use serde::Deserialize;
use utoipa::{openapi::path::{Parameter, ParameterBuilder}, IntoParams, PartialSchema};

use crate::dtos::PaginationFilter;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PaginationFilterWithOrder<T> {
	#[serde(flatten)]
	page_take: PaginationFilter,

	#[serde(default)]
	column: Option<T>,

	#[serde(default)]
	is_asc: bool,
}


impl<T> Default for PaginationFilterWithOrder<T> {
	fn default() -> Self {
		Self {
			page_take: Default::default(),
			column: None,
			is_asc: Default::default(),
		}
	}
}

impl<T> PaginationFilterWithOrder<T> {
	pub fn page(&self) -> u64 {
		self.page_take.page()
	}

	pub fn skip(&self) -> u64 {
		self.page_take.skip()
	}

	pub fn take(&self) -> u64 {
		self.page_take.take()
	}

	pub fn order_by_column(&self) -> &Option<T> {
		&self.column
	}

	pub fn order_by_is_asc(&self) -> bool {
		self.is_asc
	}
}

impl<T: PartialSchema> IntoParams for PaginationFilterWithOrder<T> {
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

		res.push(
			ParameterBuilder::new()
				.name("column")
				.parameter_in(parameter_in_provider().unwrap_or_default())
				.description(Some("Column"))
				.schema(Some(T::schema()))
				.required(utoipa::openapi::Required::False)
				.build(),
		);

		res.push(
			ParameterBuilder::new()
				.name("is_asc")
				.parameter_in(parameter_in_provider().unwrap_or_default())
				.description(Some("Ascending"))
				.schema(Some(bool::schema()))
				.required(utoipa::openapi::Required::False)
				.build(),
		);

		res
	}
}
