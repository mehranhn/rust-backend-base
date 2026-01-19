use std::borrow::Cow;

use axum::{
	body::Body,
	http::{HeaderValue, Response, header::CONTENT_TYPE},
	response::IntoResponse,
};
use utoipa::{
	PartialSchema, ToResponse, ToSchema,
	openapi::{ContentBuilder, ObjectBuilder, ResponseBuilder, Schema, Type, schema::SchemaType},
};

use crate::{error::DynError, app::errors::ErrServerError};

#[derive(Debug)]
pub struct RespServerErrorLogged {
	error: DynError,
}

impl RespServerErrorLogged {
	pub fn new(e: DynError) -> Self {
		Self { error: e }
	}
}

impl IntoResponse for RespServerErrorLogged {
	fn into_response(self) -> Response<Body> {
		tracing::error!("Server Error: \"{}\"", self.error);
		#[cfg(feature = "develop")]
		let mut res = Response::new(Body::empty());

		#[cfg(not(feature = "develop"))]
		let mut res = Response::new(Body::new(format!("{}", self.error)));

		res.headers_mut()
			.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));

		res
	}
}

impl<'r> ToResponse<'r> for RespServerErrorLogged {
	fn response() -> (
		&'r str,
		utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
	) {
		let mut r = ResponseBuilder::new().description("Server Error");

		if cfg!(feature = "develop") {
			r = r.content(
				"text/plain",
				ContentBuilder::new().schema(Some(Self::schema())).build(),
			);
		}

		("ServerError", r.build().into())
	}
}

impl PartialSchema for RespServerErrorLogged {
	fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
		utoipa::openapi::RefOr::T(Schema::Object(
			ObjectBuilder::new()
				.schema_type(SchemaType::Type(Type::String))
				.build(),
		))
	}
}

impl ToSchema for RespServerErrorLogged {
	fn name() -> std::borrow::Cow<'static, str> {
		Cow::Borrowed("ServerError")
	}
}

impl From<DynError> for RespServerErrorLogged {
	fn from(value: DynError) -> Self {
		Self { error: value }
	}
}

impl From<ErrServerError> for RespServerErrorLogged {
	fn from(value: ErrServerError) -> Self {
		Self { error: value.0 }
	}
}
