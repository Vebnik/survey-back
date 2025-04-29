use axum::routing::{post, Router};

use super::routes::*;

pub fn router() -> Router {
    Router::new()
        .route("/answer/submit", post(submit))
        .route("/answer/statistic", post(statistic))
}
