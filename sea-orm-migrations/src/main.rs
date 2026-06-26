use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(sea_orm_migrations::Migrator).await;
}
