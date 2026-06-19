use std::{net::SocketAddr, time::Duration};

use crate::{app::App, external::repo::ExRepo};
use axum::{Router, middleware::from_fn_with_state};
use axum_server::{Handle, tls_rustls::RustlsConfig};
use tokio::net::TcpListener;
use tower_http::{
	LatencyUnit,
	compression::CompressionLayer,
	trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::{
	api::state::AxumState,
	config::{ConfigApi, ListenAddress},
};

mod extractors;
mod middlewares;
pub mod routes;
mod state;

#[cfg(feature = "swagger")]
mod openapi;

mod responses;
#[cfg(feature = "embed-web-ui")]
mod web_ui;
mod utils;

fn routes<Repo: ExRepo>(state: AxumState<Repo>) -> Router {
	let r = Router::new()
		.nest(
			"/api",
			routes::routes_api().route_layer(from_fn_with_state(
				state.clone(),
				middlewares::extract_auth_data,
			)),
		)
		.with_state(state);

	#[cfg(feature = "swagger")]
	let r = {
		use utoipa::OpenApi;

		r.merge(
			utoipa_swagger_ui::SwaggerUi::new("/docs/swagger")
				.url("/docs/openapi.json", openapi::ApiDoc::openapi()),
		)
		.merge(utoipa_rapidoc::RapiDoc::new("/docs/openapi.json").path("/docs/rapidoc"))
	};

	#[cfg(feature = "embed-web-ui")]
	let r = r.merge(web_ui::web_ui_router());

	r.layer(
		TraceLayer::new_for_http()
			.make_span_with(DefaultMakeSpan::new().include_headers(false))
			.on_request(DefaultOnRequest::new().level(Level::DEBUG))
			.on_response(
				DefaultOnResponse::new()
					.level(Level::DEBUG)
					.latency_unit(LatencyUnit::Micros),
			),
	)
	.layer(CompressionLayer::new())
}

#[derive(Debug, thiserror::Error)]
pub enum ErrStart {
	#[error(transparent)]
	AxumServer(#[from] std::io::Error),
}

pub async fn start<Repo: ExRepo>(
	app: &'static App<Repo>, config: ConfigApi,
) -> Result<(), ErrStart> {
	let state = AxumState::new(app);
	let routes = routes(state);

	match config.host {
		#[cfg(unix)]
		ListenAddress::Unix(path) => {
			use tokio::net::UnixListener;

			let listener = UnixListener::bind(path.clone())?;
			tracing::info!("listening on {:?}", listener.local_addr());
			axum::serve(listener, routes)
				.with_graceful_shutdown(app.shutdown_token().cancelled_owned())
				.await?;

			let _ = tokio::fs::remove_file(path).await;
		},
		ListenAddress::Tcp {
			address,
			port,
			tls,

			#[cfg(target_os = "linux")]
			fwmark,
		} => {
			let listener = TcpListener::bind(SocketAddr::new(address, port)).await?;

			#[cfg(target_os = "linux")]
			if let Some(fw) = fwmark {
				use {
					nix::sys::socket::sockopt::Mark,
					std::{borrow::Borrow, os::fd::AsFd},
				};

				if let Err(e) =
					nix::sys::socket::setsockopt(listener.as_fd().borrow(), Mark, &(fw as u32))
				{
					tracing::error!("failed to set fwmark to {fw}, errno: {e}");
				}
			}

			let handle = Handle::new();

			let h = handle.clone();
			tokio::spawn(async move {
				app.shutdown_token().cancelled().await;
				h.graceful_shutdown(Some(Duration::from_secs(10)));
			});

			let h2 = handle.clone();
			let is_https = tls.is_some();
			tokio::spawn(async move {
				if let Some(s) = h2.listening().await {
					if is_https {
						tracing::info!("listening on https://{:?}", s);
					} else {
						tracing::info!("listening on http://{:?}", s);
					}
				}
			});

			match tls {
				Some(t) => {
					let rustls_config = RustlsConfig::from_pem_file(t.cert, t.key).await?;

					axum_server::from_tcp_rustls(listener.into_std()?, rustls_config)?
						.handle(handle)
						.serve(routes.into_make_service())
						.await?;
				},
				None => {
					axum_server::from_tcp(listener.into_std()?)?
						.handle(handle)
						.serve(routes.into_make_service())
						.await?;
				},
			}
		},
	}

	Ok(())
}
