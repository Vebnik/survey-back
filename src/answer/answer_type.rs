use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answers {
    pub after_six_months: String,
    pub before_surgery: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: i32,
    pub question: String,
    pub answers: Answers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitPayload {
    pub date: String,
    pub questions: Vec<Question>,
    pub satisfied: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticDatePayload {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticPayload {
    pub date: StatisticDatePayload,
}

#[allow(dead_code)]
pub struct SubmitQuestionPayload {
    pub submit_answer_id: String,
    pub question: String,
    pub after_six_months: String,
    pub before_surgery: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitQuestion {
    pub id: String,

    pub submit_answer_id: String,

    pub question: String,
    pub after_six_months: String,
    pub before_surgery: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitAnswer {
    pub id: String,

    pub date: String,
    pub satisfied: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
