use sqlx::SqlitePool;
use std::sync::Arc;

use crate::config::Config;

pub type Db = Arc<SqlitePool>;

#[derive(Clone)]
pub struct Ctx {
    pub db: Db,
    pub config: Config,
}
