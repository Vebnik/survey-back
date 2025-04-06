use anyhow::Context;
use std::{string::String, sync::Arc};

fn get_env(key: &str) -> String {
    dotenvy::var(key)
        .context(format!("{} must be set", key))
        .unwrap()
}

#[derive(Debug, Clone)]
pub struct Secrets {
    pub db_url: Arc<String>,
}

#[derive(Debug, Clone)]
pub struct Constants {}

#[derive(Debug, Clone)]
pub struct Config {
    pub whitelist_emails: Arc<Vec<String>>,
    pub front_url: Arc<String>,
    pub secrets: Arc<Secrets>,
    pub constants: Arc<Constants>,
}

impl Config {
    pub fn new() -> Self {
        let whitelist_emails = get_env("WHITELIST_EMAILS")
            .split(",")
            .map(|el| el.into())
            .collect::<Vec<String>>();

        Self {
            whitelist_emails: Arc::new(whitelist_emails),
            front_url: Arc::new(get_env("FRONT_URL")),
            secrets: Arc::new(Secrets {
                db_url: Arc::new(get_env("DATABASE_URL")),
            }),
            constants: Arc::new(Constants {}),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
