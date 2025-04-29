use uuid::Uuid;

use super::{Question, StatisticPayload, SubmitAnswer, SubmitPayload, SubmitQuestion};
use crate::{Db, Result};

impl SubmitAnswer {
    pub async fn get_all(db: Db) -> Result<Vec<Self>> {
        sqlx::query_as!(SubmitAnswer, r#"SELECT * from "submit_answer""#,)
            .fetch_all(db.as_ref())
            .await
            .map_err(Into::into)
    }

    pub async fn get_all_by_date(db: Db, payload: StatisticPayload) -> Result<Vec<Self>> {
        sqlx::query_as!(
            SubmitAnswer,
            r#"
                SELECT * from "submit_answer" sa
                WHERE sa.date BETWEEN $1 AND $2
            "#,
            payload.date.from,
            payload.date.to
        )
            .fetch_all(db.as_ref())
            .await
            .map_err(Into::into)
    }

    pub async fn create(db: Db, payload: SubmitPayload) -> Result<Self> {
        let id = Uuid::new_v4().to_string();

        sqlx::query_as!(
            SubmitAnswer,
            r#"
                INSERT INTO "submit_answer" ("id", "date", "satisfied")
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
            id,
            payload.date,
            payload.satisfied,
        )
        .fetch_one(db.as_ref())
        .await
        .map_err(Into::into)
    }
}

impl SubmitQuestion {
    pub async fn get_question_by_answer(db: Db, submit_answer_id: &str) -> Result<Vec<Self>> {
        sqlx::query_as!(
            SubmitQuestion,
            r#"
                SELECT * from "submit_question"
                WHERE "submit_answer_id" = $1
            "#,
            submit_answer_id
        )
        .fetch_all(db.as_ref())
        .await
        .map_err(Into::into)
    }

    pub async fn create(db: Db, payload: Question, submit_answer_id: &str) -> Result<Self> {
        let id = Uuid::new_v4().to_string();

        sqlx::query_as!(
            SubmitQuestion,
            r#"
                INSERT INTO "submit_question" ("id", "submit_answer_id", "question", "after_six_months", "before_surgery")
                VALUES ($1, $2, $3, $4, $5)
                RETURNING *
            "#,
            id,
            submit_answer_id,
            payload.question,
            payload.answers.after_six_months,
            payload.answers.before_surgery,
        )
        .fetch_one(db.as_ref())
        .await
        .map_err(Into::into)
    }
}
