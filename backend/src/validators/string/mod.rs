mod length;

use std::borrow::Cow;

use regex::Regex;
use serde_validate::Validate;
use utoipa::{
	PartialSchema, ToSchema,
	openapi::{
		ObjectBuilder, RefOr, Type,
		schema::{Schema, SchemaType},
	},
};

pub use length::StringVLength;

macro_rules! string_wrapper {
	(
        $name:ident
    ) => {
		#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		pub struct $name(String);

		impl<'__de> serde::Deserialize<'__de> for $name {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: serde::Deserializer<'__de>,
			{
				let helper = String::deserialize(deserializer)?;
				let instance = Self(helper);
				instance.validated().map_err(serde::de::Error::custom)
			}
		}

		impl $name {
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

		impl Into<String> for $name {
			fn into(self) -> String {
				self.0
			}
		}

		impl std::ops::Deref for $name {
			type Target = String;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl std::ops::DerefMut for $name {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}
	};
	(
        $name:ident
        $(, min = $min:expr)?
        $(, max = $max:expr)?
		$(, regex = $regex:expr)?
        $(,)?
    ) => {
		#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		pub struct $name(String);

		impl<'__de> serde::Deserialize<'__de> for $name {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: serde::Deserializer<'__de>,
			{
				let helper = String::deserialize(deserializer)?;
				let instance = Self(helper);
				instance.validated().map_err(serde::de::Error::custom)
			}
		}

		impl Validate for $name {
			type Error = &'static str;

			fn validate(&self) -> Result<(), Self::Error> {
                $(
                    if self.0.len() < $min {
                        return Err(concat!("length must be >= ", stringify!($min)));
                    }
                )?

                $(
                    if self.0.len() > $max {
                        return Err(concat!("length must be <= ", stringify!($max)));
                    }
                )?

                $(
                    if $regex.is_match(&self.0) {
                        return Err(concat!("does not match required pattern", stringify!($regex)));
                    }
                )?

				Ok(())
			}

			fn validated(self) -> Result<Self, Self::Error> {
				self.validate()?;
				Ok(self)
			}
		}

		impl PartialSchema for $name {
			fn schema() -> RefOr<Schema> {
				let mut s = ObjectBuilder::new()
					.schema_type(SchemaType::Type(Type::String));

                $(
					s = s.min_length(Some($min));
                )?

                $(
					s = s.max_length(Some($max));
                )?

                $(
					s = s.pattern(Some($regex.as_str()));
                )?

				s.build().into()
			}
		}

		impl ToSchema for $name {
			fn name() -> Cow<'static, str> {
				Cow::Borrowed(stringify!($name))
			}
		}

		impl $name {
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

		impl Into<String> for $name {
			fn into(self) -> String {
				self.0
			}
		}

		impl std::ops::Deref for $name {
			type Target = String;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl std::ops::DerefMut for $name {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}
	};
}

string_wrapper!(StringVUsername, min = 3, max = 255);
string_wrapper!(StringVPassword, min = 5, max = 2048);
string_wrapper!(
	StringVPhone,
	min = 3,
	max = 17,
	regex = unsafe { Regex::new(r"^\+?[1-9]\d{7,14}$").unwrap_unchecked() }
);
string_wrapper!(StringVShort, max = 50);
string_wrapper!(StringVMedium, max = 255);
string_wrapper!(StringVLong, max = 2048);

string_wrapper!(StringVEmail);
string_wrapper!(StringVUrl);

impl Validate for StringVEmail {
	type Error = &'static str;

	fn validate(&self) -> Result<(), Self::Error> {
		use validator::ValidateEmail;

		if self.0.len() > 255 {
			return Err("length must be <= 255");
		}

		if self.validate_email() {
			Ok(())
		} else {
			Err("must be a valid email")
		}
	}

	fn validated(self) -> Result<Self, Self::Error> {
		self.validate()?;
		Ok(self)
	}
}

impl PartialSchema for StringVEmail {
	fn schema() -> RefOr<Schema> {
		ObjectBuilder::new()
			.schema_type(SchemaType::Type(Type::String))
			.max_length(Some(255))
			.build()
			.into()
	}
}

impl ToSchema for StringVEmail {
	fn name() -> Cow<'static, str> {
		Cow::Borrowed("StringEmail")
	}
}

impl Validate for StringVUrl {
	type Error = &'static str;

	fn validate(&self) -> Result<(), Self::Error> {
		use validator::ValidateUrl;

		if self.0.len() > 2048 {
			return Err("length must be <= 2048");
		}

		if self.validate_url() {
			Ok(())
		} else {
			Err("must be a valid email")
		}
	}

	fn validated(self) -> Result<Self, Self::Error> {
		self.validate()?;
		Ok(self)
	}
}

impl PartialSchema for StringVUrl {
	fn schema() -> RefOr<Schema> {
		ObjectBuilder::new()
			.schema_type(SchemaType::Type(Type::String))
			.max_length(Some(2048))
			.build()
			.into()
	}
}

impl ToSchema for StringVUrl {
	fn name() -> Cow<'static, str> {
		Cow::Borrowed("StringUrl")
	}
}
