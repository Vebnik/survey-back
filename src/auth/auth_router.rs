use axum::{
    middleware,
    routing::{get, Router},
};

use super::{jwt_auth, routes::*};

pub fn router() -> Router {
    Router::new()
        .route("/auth", get(auth))
        .layer(middleware::from_fn(jwt_auth))
}
