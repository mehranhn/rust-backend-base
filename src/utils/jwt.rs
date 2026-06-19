use jsonwebtoken::{
	Algorithm, DecodingKey, EncodingKey, Header, Validation, encode, get_current_timestamp,
};
use serde::{Deserialize, Serialize};
use time::Duration;

use crate::dtos::AuthData;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
struct AuthToken {
	pub exp: u64,

	#[serde(flatten)]
	pub data: AuthData,
}

impl AuthToken {
	fn new(exp: u64, data: AuthData) -> Self {
		Self { exp, data }
	}
}

pub fn encode_to_jwt(
	data: AuthData, secret: &str, valid_duration: Duration,
) -> Result<String, jsonwebtoken::errors::Error> {
	let auth_token = AuthToken::new(get_current_timestamp() + valid_duration.whole_seconds() as u64, data);
	let token = encode(
		&Header::default(),
		&auth_token,
		&EncodingKey::from_secret(secret.as_bytes()),
	)?;

	Ok(token)
}

pub fn decode_from_jwt(token: &str, secret: &str) -> Result<AuthData, Option<AuthData>> {
	let mut validations = Validation::new(Algorithm::HS256);
	validations.validate_exp = false;

	match jsonwebtoken::decode::<AuthToken>(
		token,
		&DecodingKey::from_secret(secret.as_bytes()),
		&validations,
	) {
		Ok(o) => {
			let data = o.claims;
			if data.exp <= get_current_timestamp() {
				Err(Some(data.data))
			} else {
				Ok(data.data)
			}
		},
		Err(_) => Err(None),
	}
}
