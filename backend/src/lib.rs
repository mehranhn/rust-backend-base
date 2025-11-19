#![allow(dead_code)]
#![allow(unused_imports)]

use axum::response::IntoResponse;
use axum::http::status::StatusCode;

use crate::error::DynError;

mod api;
mod config;
mod constants;
mod dtos;
mod error;
mod external;
mod modules;
mod utils;

#[derive(Debug, utoipa::IntoResponses)]
pub enum EEEE {
	#[response(status = NOT_FOUND, description = "not found")]
	NotFound,

	#[response(status = 500)]
	ServerError(String),
}
