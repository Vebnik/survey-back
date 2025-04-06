use anyhow::Context;
use sqlx::sqlite::SqlitePoolOptions;
use std::{env, sync::Arc};

mod auth;
mod config;
mod error;
mod router;
mod types;
mod user;

pub use error::*;
pub use types::{Ctx, Db};

use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // configure environment
    let config = Config::new();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();
    log::info!("Logger initialized");

    log::info!("log::info");
    log::debug!("log::debug");
    log::warn!("log::warn");
    log::error!("log::error");
    log::trace!("log::trace");

    log::info!("Connecting to database");
    let db = SqlitePoolOptions::new()
        .max_connections(20)
        .connect(config.secrets.db_url.as_ref())
        .await
        .context("Failed to connect to DATABASE_URL")?;

    log::info!("Database connected");

    log::info!("Running migrations");
    sqlx::migrate!().set_locking(false).run(&db).await?;

    log::info!("Migrations completed");

    let db = Arc::new(db);

    let ctx = Ctx { db, config };

    router::serve(ctx).await
}
