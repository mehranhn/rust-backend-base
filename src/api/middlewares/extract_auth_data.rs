use crate::{
	api::utils::jwt_token_into_cookie,
	dtos::AuthData,
	external::{memory::ExMemory, repo::ExRepo},
	utils::{decode_from_jwt, encode_to_jwt},
};
use axum::{
	extract::{Request, State},
	http::{HeaderValue, header::SET_COOKIE},
	middleware::Next,
	response::Response,
};
use axum_extra::extract::CookieJar;

use crate::api::state::AxumState;

pub async fn extract_auth_data<D: ExRepo, M: ExMemory>(
	State(s): State<AxumState<D, M>>, cookie: CookieJar, mut request: Request, next: Next,
) -> Response {
	let mut new_token: Option<(AuthData, String, bool)> = None;
	let token: Option<(&str, bool)> = match cookie.get("auth") {
		Some(auth) => Some((auth.value(), true)),
		None => match request.headers().get(axum::http::header::AUTHORIZATION) {
			Some(authorization) => match authorization.to_str() {
				Ok(authorization) => {
					if authorization.starts_with("Bearer ") {
						Some((authorization.split_at(7).1, false))
					} else {
						None
					}
				},
				Err(_) => None,
			},
			None => None,
		},
	};

	if let Some((token, is_from_cookie)) = token {
		let decoded_token = decode_from_jwt(token, &s.app.config().jwt_secret);
		match decoded_token {
			Ok(auth_data) => {
				if let Ok(is_blacklist) = s
					.app
					.session_blacklist_is_blacklist(auth_data.session_id)
					.await && !is_blacklist
				{
					request.extensions_mut().insert(auth_data);
				}
			},
			Err(Some(auth_data)) => match s.app.renew_session(auth_data.session_id).await {
				Ok(new_auth_data) => {
					new_token = Some((
						auth_data.clone(),
						encode_to_jwt(
							new_auth_data.clone(),
							&s.app.config().jwt_secret,
							s.app.config().jwt_exp_after,
						)
						.unwrap(),
						is_from_cookie,
					));
					request.extensions_mut().insert(new_auth_data);
				},
				Err(_) => {
					// request.extensions_mut().insert(Option::<AuthData>::None);
				},
			},
			Err(None) => {
				// request.extensions_mut().insert(Option::<AuthData>::None);
			},
		};
	} else {
		// request.extensions_mut().insert(Option::<AuthData>::None);
	}

	let mut response = next.run(request).await;

	if let Some((auth_data, new_token, is_from_cookie)) = new_token {
		if is_from_cookie {
			if let Ok(hv) = HeaderValue::from_str(
				jwt_token_into_cookie(new_token.as_str(), auth_data.expire_at).as_str(),
			) {
				response.headers_mut().insert(SET_COOKIE, hv);
			}
		} else {
			#[allow(clippy::collapsible_else_if)]
			if let Ok(hv) = HeaderValue::from_str(format!("Bearer {}", new_token.as_str()).as_str())
			{
				response.headers_mut().insert("X-New-Token", hv);
			}
		}
	}

	response
}
