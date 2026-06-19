use axum_extra::extract::cookie::{Cookie, Expiration, SameSite};
use time::OffsetDateTime;

pub fn jwt_token_into_cookie(token: &str, expire_date: Option<OffsetDateTime>) -> String {
	let mut cb = Cookie::build(("auth", token))
		.http_only(true)
		.same_site(SameSite::Strict)
		.path("/api");

	match expire_date {
		Some(d) => cb = cb.expires(Expiration::DateTime(d)),
		None => cb = cb.permanent(),
	}

	cb.build().to_string()
}
