#![allow(dead_code)]
use std::borrow::Cow;

use serde_validate::Validate;
use utoipa::{
	PartialSchema, ToSchema,
	openapi::{RefOr, Schema},
};

#[derive(Debug, Clone)]
pub struct VecVLength<T, const MIN: usize = { usize::MIN }, const MAX: usize = { usize::MAX }>(
	Vec<T>,
);

impl<T, const MIN: usize, const MAX: usize> Validate for VecVLength<T, MIN, MAX> {
	type Error = &'static str;

	fn validate(&self) -> Result<(), Self::Error> {
		if self.0.len() < MIN {
			return Err(concat!("array size must be >= ", stringify!(MIN)));
		}

		if self.0.len() > MAX {
			return Err(concat!("array size must be <= ", stringify!(MAX)));
		}

		Ok(())
	}

	fn validated(self) -> Result<Self, Self::Error> {
		self.validate()?;
		Ok(self)
	}
}

impl<T: ToSchema, const MIN: usize, const MAX: usize> PartialSchema for VecVLength<T, MIN, MAX>
where
	Vec<T>: ToSchema,
{
	fn schema() -> RefOr<Schema> {
		let a = Vec::<T>::schema();
		if let RefOr::T(Schema::Array(mut array)) = a {
			if MIN != usize::MIN {
				array.min_items = Some(MIN);
			}

			if MAX != usize::MAX {
				array.max_items = Some(MAX);
			}

			array.into()
		} else {
			a
		}
	}
}

impl<T: ToSchema, const MIN: usize, const MAX: usize> ToSchema for VecVLength<T, MIN, MAX>
where
	VecVLength<T, MIN, MAX>: PartialSchema,
{
	fn name() -> Cow<'static, str> {
		Cow::Owned(format!("VecVLength_{MIN}_{MAX}"))
	}
}

impl<T, const MIN: usize, const MAX: usize> VecVLength<T, MIN, MAX> {
	pub fn new(value: Vec<T>) -> Result<Self, &'static str> {
		Self(value).validated()
	}

	/// # SAFETY
	///
	/// you must insure that provided value is already validated
	pub unsafe fn new_unchecked(value: Vec<T>) -> Self {
		Self(value)
	}

	pub fn into_inner(self) -> Vec<T> {
		self.0
	}
}

impl<T, const MIN: usize, const MAX: usize> Into<Vec<T>> for VecVLength<T, MIN, MAX> {
	fn into(self) -> Vec<T> {
		self.0
	}
}

impl<T, const MIN: usize, const MAX: usize> std::ops::Deref for VecVLength<T, MIN, MAX> {
	type Target = Vec<T>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T, const MIN: usize, const MAX: usize> std::ops::DerefMut for VecVLength<T, MIN, MAX> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<'__de, T: serde::Deserialize<'__de>, const MIN: usize, const MAX: usize>
	serde::Deserialize<'__de> for VecVLength<T, MIN, MAX>
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'__de>,
	{
		let helper = Vec::<T>::deserialize(deserializer)?;
		let instance = Self(helper);
		instance.validated().map_err(serde::de::Error::custom)
	}
}
