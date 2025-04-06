use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde_json::json;

use crate::{user::User, Ctx, Result};

pub async fn auth(
    Extension(_ctx): Extension<Ctx>,
    Extension(user): Extension<User>,
) -> Result<Response> {
    log::debug!("user: {:#?}", user);

    Ok((StatusCode::OK, Json(json!(user))).into_response())
}
