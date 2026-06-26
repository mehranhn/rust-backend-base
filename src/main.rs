use base64::{Engine, engine::general_purpose::STANDARD};
use clap::Parser;
use rust_backend_base::{
	app::App,
	external::{
		memory::implementations::ExMemoryPapaya, repo::implementations::ExRepoImplSeaOrmPg,
	},
	utils,
};
use tokio_util::sync::CancellationToken;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::cli::{Commands, HashPasswordCommands};

mod cli;

async fn terminate_on_intrupt(ct: CancellationToken) {
	if cfg!(unix) {
		let mut sigterm =
			tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).unwrap();
		let mut sigint =
			tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt()).unwrap();

		tokio::select! {
			biased;

			_ = sigterm.recv() => {
				ct.cancel();
			}

			_ = sigint.recv() => {
				ct.cancel();
			}

			_ = ct.cancelled() => {}
		}
	} else {
		tokio::select! {
			biased;

			_ = tokio::signal::ctrl_c() => {
				ct.cancel();
			}

			_ = ct.cancelled() => {}
		}
	}
}

async fn run() {
	#[cfg(debug_assertions)]
	if let Err(err) = dotenvy::dotenv() {
		println!("dotenv: {err}");
	}

	let c = cli::Cli::parse();
	match c.command {
		Commands::Run {
			config: config_path,
		} => {
			let config = rust_backend_base::config::read_config(config_path)
				.await
				.unwrap();
			tracing_subscriber::registry()
				// .with(tracing_subscriber::EnvFilter::new(format!(
				// 	"sqlx={0},tower_http={0},rust_backend_base={0}",
				// 	config.log_level.as_ref()
				// )))
				.with(tracing_subscriber::EnvFilter::new(
					config.log_level.as_ref(),
				))
				.with(tracing_subscriber::fmt::layer())
				.init();
			let db = ExRepoImplSeaOrmPg::new(&config.database_url).await.unwrap();
			let memory = ExMemoryPapaya::new();
			let shutdown_token = CancellationToken::new();
			let app = App::new(config.app.clone(), shutdown_token.clone(), db, memory);
			let app = Box::leak(Box::new(app));
			app.init().await.unwrap();

			let ct = shutdown_token.clone();
			tokio::spawn(async move {
				terminate_on_intrupt(ct).await;
			});

			rust_backend_base::api::start(app, config.api)
				.await
				.unwrap();
		},
		Commands::HashPassword(h) => match h {
			HashPasswordCommands::Sha256(p) => {
				let hashed = utils::hash_password(p.username.as_str(), p.password.as_str());
				let string = STANDARD.encode(hashed);
				println!("{string}");
			},
		},
	}
}

#[tokio::main]
async fn main() {
	run().await
}
