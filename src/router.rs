use anyhow::Context;
use axum::{
    body::Body,
    http::{HeaderValue, Request, StatusCode},
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use http::HeaderName;
use reqwest::Method;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

use crate::{auth, answer, Ctx};

const PORT: u16 = 4042;

pub async fn health() -> &'static str {
    "OK"
}

pub fn main_router() -> Router {
    Router::new()
        // Health check
        .route("/health", get(health))
}

pub fn app(ctx: Ctx) -> Router {
    let front_url = ctx.config.front_url.clone();

    Router::new()
        .merge(main_router())
        .merge(auth::router())
        .merge(answer::router())
        .fallback(fallback)
        .layer(Extension(ctx))
        .layer(
            CorsLayer::new()
                .allow_origin(vec![front_url.parse::<HeaderValue>().unwrap()])
                .allow_methods(vec![Method::GET, Method::POST])
                .allow_headers(vec![
                    http::header::CONTENT_TYPE,
                    HeaderName::from_static("x-token"),
                ])
                .allow_credentials(true),
        )
}

pub async fn serve(ctx: Ctx) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .unwrap();
    log::info!("Starting server on {}", PORT);
    axum::serve(
        listener,
        app(ctx).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .context("Failed to serve API")
}

async fn fallback(req: Request<Body>) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        format!("Oops! No route found for {}", req.uri()),
    )
}
