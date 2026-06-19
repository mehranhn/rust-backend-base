use std::borrow::Cow;

use serde::Deserialize;
use utoipa::{
	PartialSchema, ToSchema,
	openapi::{
		ObjectBuilder, OneOfBuilder, RefOr, Type,
		schema::{Schema, SchemaType},
	},
};

#[derive(Debug, Copy, Clone, Default)]
pub enum NullUndefinedValue<T> {
	Some(T),
	Null,
	#[default]
	Undefined,
}

impl<'de, T> Deserialize<'de> for NullUndefinedValue<T>
where
	T: Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let opt = Option::<T>::deserialize(deserializer)?;
		Ok(match opt {
			Some(v) => NullUndefinedValue::Some(v),
			None => NullUndefinedValue::Null,
		})
	}
}

impl<T: PartialSchema> PartialSchema for NullUndefinedValue<T> {
	fn schema() -> RefOr<Schema> {
		let inner = T::schema();

		RefOr::T(
			OneOfBuilder::new()
				.item(inner)
				.item(RefOr::T(Schema::Object(
					ObjectBuilder::new()
						.schema_type(SchemaType::Type(Type::Null))
						.build(),
				)))
				.build()
				.into(),
		)
	}
}

impl<T: ToSchema> ToSchema for NullUndefinedValue<T> {
	fn name() -> Cow<'static, str> {
		Cow::Borrowed("NullUndefinedValue")
	}

	fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
		<T as ToSchema>::schemas(schemas);
	}
}
