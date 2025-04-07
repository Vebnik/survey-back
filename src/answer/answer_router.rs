use axum::routing::{get, post, Router};

use super::routes::*;

pub fn router() -> Router {
    Router::new()
        .route("/answer/submit", post(submit))
        .route("/answer/statistic", get(statistic))
    // .route("/answer/statistic", post(statistic).layer(middleware::from_fn(jwt_auth)))
}
