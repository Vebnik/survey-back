use axum::{
    extract,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde_json::json;

use crate::{
    answer::{Question, SubmitAnswer, SubmitPayload},
    Ctx, Result,
};

pub async fn submit(
    Extension(ctx): Extension<Ctx>,
    extract::Json(payload): extract::Json<SubmitPayload>,
) -> Result<Response> {
    let submit_answer = SubmitAnswer::create(ctx.db.clone(), payload).await?;
    let questions = serde_json::from_str::<Vec<Question>>(&submit_answer.questions)?;

    log::debug!("{questions:#?}");

    Ok((StatusCode::OK, Json(json!(submit_answer))).into_response())
}
