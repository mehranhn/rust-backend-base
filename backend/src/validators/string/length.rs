#![allow(dead_code)]
use std::borrow::Cow;

use serde_validate::Validate;
use utoipa::{openapi::{schema::SchemaType, ObjectBuilder, RefOr, Schema, Type}, PartialSchema, ToSchema};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringVLength<const MIN: u32 = { u32::MIN }, const MAX: u32 = { u32::MAX }>(String);

impl<const MIN: u32, const MAX: u32> Validate for StringVLength<MIN, MAX> {
	type Error = &'static str;

	fn validate(&self) -> Result<(), Self::Error> {
		if self.0.len() < MIN as usize {
			return Err(concat!("length must be >= ", stringify!(MIN)));
		}

		if self.0.len() > MAX as usize {
			return Err(concat!("length must be <= ", stringify!(MAX)));
		}

		Ok(())
	}

	fn validated(self) -> Result<Self, Self::Error> {
		self.validate()?;
		Ok(self)
	}
}

impl<const MIN: u32, const MAX: u32> PartialSchema for StringVLength<MIN, MAX> {
	fn schema() -> RefOr<Schema> {
		ObjectBuilder::new()
			.schema_type(SchemaType::Type(Type::String))
			.min_length(Some(MIN as usize))
			.max_length(Some(MAX as usize))
			.build()
			.into()
	}
}

impl<const MIN: u32, const MAX: u32> ToSchema for StringVLength<MIN, MAX> {
	fn name() -> Cow<'static, str> {
		Cow::Owned(format!("StringLength_{MIN}_{MAX}"))
	}
}

impl<const MIN: u32, const MAX: u32> StringVLength<MIN, MAX> {
	pub fn new(value: String) -> Result<Self, &'static str> {
		Self(value).validated()
	}

	/// # SAFETY
	///
	/// you must insure that provided value is already validated
	pub unsafe fn new_unchecked(value: String) -> Self {
		Self(value)
	}

	pub fn into_inner(self) -> String {
		self.0
	}
}

impl<const MIN: u32, const MAX: u32> Into<String> for StringVLength<MIN, MAX> {
	fn into(self) -> String {
		self.0
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Deref for StringVLength<MIN, MAX> {
	type Target = String;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::DerefMut for StringVLength<MIN, MAX> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<'__de, const MIN: u32, const MAX: u32> serde::Deserialize<'__de> for StringVLength<MIN, MAX> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'__de>,
	{
		let helper = String::deserialize(deserializer)?;
		let instance = Self(helper);
		instance.validated().map_err(serde::de::Error::custom)
	}
}
