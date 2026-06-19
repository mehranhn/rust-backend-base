#![allow(dead_code)]
use std::borrow::Cow;

use serde_validate::Validate;
use utoipa::{
	PartialSchema, ToSchema,
	openapi::{RefOr, Schema},
};

macro_rules! number_wrapper {
	(
        $name:ident,
		$type:ty
    ) => {
		#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		pub struct $name<const MIN: $type = { <$type>::MIN }, const MAX: $type = { <$type>::MAX }>(
			$type,
		);

		impl<const MIN: $type, const MAX: $type> Validate for $name<MIN, MAX> {
			type Error = &'static str;

			fn validate(&self) -> Result<(), Self::Error> {
				if self.0 < MIN {
					return Err(concat!("value must be >= ", stringify!(MIN)));
				}

				if self.0 > MAX {
					return Err(concat!("value must be <= ", stringify!(MAX)));
				}

				Ok(())
			}

			fn validated(self) -> Result<Self, Self::Error> {
				self.validate()?;
				Ok(self)
			}
		}

		impl<const MIN: $type, const MAX: $type> PartialSchema for $name<MIN, MAX> {
			fn schema() -> RefOr<Schema> {
				let a = <$type>::schema();
				if let RefOr::T(Schema::Object(mut object)) = a {
					if MIN != <$type>::MIN {
						object.minimum = Some(MIN.into());
					}

					if MAX != <$type>::MAX {
						object.maximum = Some(MAX.into());
					}

					object.into()
				} else {
					a
				}
			}
		}

		impl<const MIN: $type, const MAX: $type> ToSchema for $name<MIN, MAX> {
			fn name() -> Cow<'static, str> {
				Cow::Owned(format!("{}_{MIN}_{MAX}", stringify!($name)))
			}
		}

		impl<const MIN: $type, const MAX: $type> $name<MIN, MAX> {
			pub fn new(value: $type) -> Result<Self, &'static str> {
				Self(value).validated()
			}

			/// # SAFETY
			///
			/// you must insure that provided value is already validated
			pub unsafe fn new_unchecked(value: $type) -> Self {
				Self(value)
			}

			pub fn into_inner(self) -> $type {
				self.0
			}
		}

		impl<const MIN: $type, const MAX: $type> Into<$type> for $name<MIN, MAX> {
			fn into(self) -> $type {
				self.0
			}
		}

		impl<const MIN: $type, const MAX: $type> std::ops::Deref for $name<MIN, MAX> {
			type Target = $type;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl<const MIN: $type, const MAX: $type> std::ops::DerefMut for $name<MIN, MAX> {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}

		impl<'__de, const MIN: $type, const MAX: $type> serde::Deserialize<'__de>
			for $name<MIN, MAX>
		{
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: serde::Deserializer<'__de>,
			{
				let helper = <$type>::deserialize(deserializer)?;
				let instance = Self(helper);
				instance.validated().map_err(serde::de::Error::custom)
			}
		}
	};
}

number_wrapper!(U8VNumberBetween, u8);
number_wrapper!(U16VNumberBetween, u16);
number_wrapper!(U32VNumberBetween, u32);
number_wrapper!(U64VNumberBetween, u64);
number_wrapper!(USizeVNumberBetween, usize);
number_wrapper!(I8VNumberBetween, i8);
number_wrapper!(I16VNumberBetween, i16);
number_wrapper!(I32VNumberBetween, i32);
number_wrapper!(I64VNumberBetween, i64);
number_wrapper!(ISizeVNumberBetween, isize);
