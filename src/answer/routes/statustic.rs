use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde_json::json;

use crate::{answer::SubmitAnswer, Ctx, Result};

pub async fn statistic(Extension(ctx): Extension<Ctx>) -> Result<Response> {
    let submit_answers = SubmitAnswer::get_all(ctx.db.clone()).await?;

    Ok((StatusCode::OK, Json(json!(submit_answers))).into_response())
}
