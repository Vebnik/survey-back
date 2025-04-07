use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answers {
    after_six_months: String,
    before_surgery: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    id: i32,
    question: String,
    answers: Answers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitPayload {
    pub date: String,
    pub questions: Vec<Question>,
    pub satisfied: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitAnswer {
    pub id: String,

    pub date: String,
    pub questions: String,
    pub satisfied: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
