use serde::Deserialize;
use utoipa::ToSchema;

use crate::{dtos::NullUndefinedValue, validators::{StringVEmail, StringVPhone}};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AdminUpdateDto {
	#[schema(required = false, value_type = String)]
	pub username: Option<String>,

	#[schema(value_type = Option<StringVPhone>)]
	#[serde(default)]
	pub phone: NullUndefinedValue<StringVPhone>,

	#[schema(value_type = Option<StringVEmail>)]
	#[serde(default)]
	pub email: NullUndefinedValue<StringVEmail>,
}
