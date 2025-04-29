use axum::{
    extract, http::StatusCode, response::{IntoResponse, Response}, Extension, Json
};
use serde_json::json;

use crate::{
    answer::{SubmitAnswer, SubmitQuestion, StatisticPayload},
    Ctx, Result,
};

pub async fn statistic(
    Extension(ctx): Extension<Ctx>,
    extract::Json(payload): extract::Json<StatisticPayload>,
) -> Result<Response> {
    let submit_answers = SubmitAnswer::get_all_by_date(ctx.db.clone(), payload).await?;

    let mut values: Vec<serde_json::Value> = Vec::with_capacity(submit_answers.len());

    for answer in submit_answers {
        let questions = SubmitQuestion::get_question_by_answer(ctx.db.clone(), &answer.id).await?;

        let data = json!({
            "answer": answer,
            "questions": questions,
        });

        values.push(data);
    }

    Ok((StatusCode::OK, Json(json!(values))).into_response())
}
