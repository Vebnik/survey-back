use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde_json::json;

use crate::{user::User, Ctx, Result};

pub async fn me(
    Extension(_ctx): Extension<Ctx>,
    Extension(user): Extension<User>,
) -> Result<Response> {
    Ok((StatusCode::OK, Json(json!(user))).into_response())
}
