use axum::{
    extract,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde_json::json;

use crate::{
    answer::{SubmitAnswer, SubmitPayload, SubmitQuestion},
    Ctx, Result,
};

pub async fn submit(
    Extension(ctx): Extension<Ctx>,
    extract::Json(payload): extract::Json<SubmitPayload>,
) -> Result<Response> {
    let submit_answer = SubmitAnswer::create(ctx.db.clone(), payload.clone()).await?;

    for question in payload.questions {
        SubmitQuestion::create(ctx.db.clone(), question, &submit_answer.id).await?;
    }

    Ok((StatusCode::OK, Json(json!(submit_answer))).into_response())
}
