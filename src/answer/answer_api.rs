use uuid::Uuid;

use super::{SubmitAnswer, SubmitPayload};
use crate::{Db, Result};

impl SubmitAnswer {
    pub async fn get_all(db: Db) -> Result<Vec<Self>> {
        sqlx::query_as!(SubmitAnswer, r#"SELECT * from "submit_answer""#,)
            .fetch_all(db.as_ref())
            .await
            .map_err(Into::into)
    }

    pub async fn create(db: Db, payload: SubmitPayload) -> Result<Self> {
        let questions = serde_json::to_string(&payload.questions)?;
        let id = Uuid::new_v4().to_string();

        sqlx::query_as!(
            SubmitAnswer,
            r#"
                INSERT INTO "submit_answer" ("id", "date", "questions", "satisfied")
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
            id,
            payload.date,
            questions,
            payload.satisfied,
        )
        .fetch_one(db.as_ref())
        .await
        .map_err(Into::into)
    }
}
