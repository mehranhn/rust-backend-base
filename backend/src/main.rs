use backend::{
	dtos::SeedDto,
	external::repo::{ExRepo, implementations::ExRepoImplSeaQueryPg},
	app::App,
	utils,
};
use base64::{Engine, engine::general_purpose::STANDARD};
use clap::Parser;
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
			let config = backend::config::read_config(config_path).await.unwrap();
			tracing_subscriber::registry()
				.with(tracing_subscriber::EnvFilter::new(format!(
					"sqlx={0},tower_http={0},backend={0}",
					config.log_level.as_ref()
				)))
				.with(tracing_subscriber::fmt::layer())
				.init();
			let db = ExRepoImplSeaQueryPg::new(&config.database_url)
				.await
				.unwrap();
			db.run_migrations().await.unwrap();
			db.seed(SeedDto {
				super_admin_username: &config.app.super_admin_username,
				super_admin_hashed_password: &config.app.super_admin_hashed_password,
			})
			.await
			.unwrap();
			let shutdown_token = CancellationToken::new();
			let app = App::new(config.app.clone(), shutdown_token.clone(), db);
			let app = Box::leak(Box::new(app));
			app.init().await.unwrap();

			let ct = shutdown_token.clone();
			tokio::spawn(async move {
				terminate_on_intrupt(ct).await;
			});

			backend::api::start(app, config.api).await.unwrap();
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
