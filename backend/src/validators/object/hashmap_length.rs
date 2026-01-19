#![allow(dead_code)]
use std::{borrow::Cow, collections::HashMap, hash::Hash};

use serde_validate::Validate;
use utoipa::{
	PartialSchema, ToSchema,
	openapi::{RefOr, Schema},
};

#[derive(Debug, Clone)]
pub struct HashMapVLength<
	K,
	V,
	const MIN: usize = { usize::MIN },
	const MAX: usize = { usize::MAX },
>(HashMap<K, V>);

impl<K, V, const MIN: usize, const MAX: usize> Validate for HashMapVLength<K, V, MIN, MAX> {
	type Error = &'static str;

	fn validate(&self) -> Result<(), Self::Error> {
		if self.0.len() < MIN {
			return Err(concat!("map properties must be >= ", stringify!(MIN)));
		}

		if self.0.len() > MAX {
			return Err(concat!("map properties must be <= ", stringify!(MAX)));
		}

		Ok(())
	}

	fn validated(self) -> Result<Self, Self::Error> {
		self.validate()?;
		Ok(self)
	}
}

impl<K: ToSchema, V: ToSchema, const MIN: usize, const MAX: usize> PartialSchema
	for HashMapVLength<K, V, MIN, MAX>
where
	HashMap<K, V>: ToSchema,
{
	fn schema() -> RefOr<Schema> {
		let a = HashMap::<K, V>::schema();
		if let RefOr::T(Schema::Object(mut object)) = a {
			if MIN != usize::MIN {
				object.min_properties = Some(MIN);
			}

			if MAX != usize::MAX {
				object.max_properties = Some(MAX);
			}

			object.into()
		} else {
			a
		}
	}
}

impl<K: ToSchema, V: ToSchema, const MIN: usize, const MAX: usize> ToSchema
	for HashMapVLength<K, V, MIN, MAX>
where
	HashMap<K, V>: ToSchema,
{
	fn name() -> Cow<'static, str> {
		Cow::Owned(format!("HashMapVLength_{MIN}_{MAX}"))
	}
}

impl<K, V, const MIN: usize, const MAX: usize> HashMapVLength<K, V, MIN, MAX> {
	pub fn new(value: HashMap<K, V>) -> Result<Self, &'static str> {
		Self(value).validated()
	}

	/// # SAFETY
	///
	/// you must insure that provided value is already validated
	pub unsafe fn new_unchecked(value: HashMap<K, V>) -> Self {
		Self(value)
	}

	pub fn into_inner(self) -> HashMap<K, V> {
		self.0
	}
}

impl<K, V, const MIN: usize, const MAX: usize> Into<HashMap<K, V>>
	for HashMapVLength<K, V, MIN, MAX>
{
	fn into(self) -> HashMap<K, V> {
		self.0
	}
}

impl<K, V, const MIN: usize, const MAX: usize> std::ops::Deref for HashMapVLength<K, V, MIN, MAX> {
	type Target = HashMap<K, V>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<K, V, const MIN: usize, const MAX: usize> std::ops::DerefMut
	for HashMapVLength<K, V, MIN, MAX>
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<
	'__de,
	K: serde::Deserialize<'__de> + Hash + Eq,
	V: serde::Deserialize<'__de>,
	const MIN: usize,
	const MAX: usize,
> serde::Deserialize<'__de> for HashMapVLength<K, V, MIN, MAX>
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'__de>,
	{
		let helper = HashMap::<K, V>::deserialize(deserializer)?;
		let instance = Self(helper);
		instance.validated().map_err(serde::de::Error::custom)
	}
}
