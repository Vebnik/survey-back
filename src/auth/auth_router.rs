use axum::{
    middleware,
    routing::{get, post, Router},
};

use super::{jwt_auth, routes::*};

pub fn router() -> Router {
    Router::new()
        .route("/auth", post(auth))
        .route("/me", get(me).layer(middleware::from_fn(jwt_auth)))
}
