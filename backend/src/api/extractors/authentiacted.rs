use std::{
	future::ready,
	marker::PhantomData,
	ops::{Deref, DerefMut},
};

use axum::{body::Body, extract::FromRequestParts, http::request::Parts, response::Response};

use crate::{dtos::AuthData, permission::Permission};

pub trait Authenticatable {
	fn authenticate(auth_data: &AuthData) -> bool;
}

impl Authenticatable for () {
	fn authenticate(_auth_data: &AuthData) -> bool {
		true
	}
}

impl<P: Permission> Authenticatable for P {
	fn authenticate(auth_data: &AuthData) -> bool {
		auth_data.role.has_permission(P::permission())
	}
}

macro_rules! impl_authenticatable_for_tuple {
    ($($P:ident),+) => {
        impl<$($P: Permission),+> Authenticatable for ($($P),+) {
            fn authenticate(auth_data: &AuthData) -> bool {
                false $(|| auth_data.role.has_permission($P::permission()) )+
            }
        }
    };
}

impl_authenticatable_for_tuple!(P1, P2);
impl_authenticatable_for_tuple!(P1, P2, P3);
impl_authenticatable_for_tuple!(P1, P2, P3, P4);
impl_authenticatable_for_tuple!(P1, P2, P3, P4, P5);
impl_authenticatable_for_tuple!(P1, P2, P3, P4, P5, P6);
impl_authenticatable_for_tuple!(P1, P2, P3, P4, P5, P6, P7);
impl_authenticatable_for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8);
impl_authenticatable_for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9);
impl_authenticatable_for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
impl_authenticatable_for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
impl_authenticatable_for_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12);

pub struct Authenticated<P: Authenticatable = ()>(AuthData, PhantomData<P>);

impl<P: Authenticatable> Deref for Authenticated<P> {
	type Target = AuthData;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<P: Authenticatable> DerefMut for Authenticated<P> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<S: Send + Sync, P: Authenticatable + Send + Sync> FromRequestParts<S> for Authenticated<P> {
	type Rejection = Response;

	fn from_request_parts(
		parts: &mut Parts, _state: &S,
	) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
		match parts.extensions.get::<AuthData>() {
			Some(a) => {
				if !P::authenticate(a) {
					let mut res = Response::new(Body::empty());
					*res.status_mut() = axum::http::status::StatusCode::FORBIDDEN;
					return ready(Err(res));
				}

				ready(Ok(Self(a.clone(), PhantomData)))
			},
			None => {
				let mut res = Response::new(Body::empty());
				*res.status_mut() = axum::http::status::StatusCode::UNAUTHORIZED;
				ready(Err(res))
			},
		}
	}
}
