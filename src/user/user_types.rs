use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,

    /// user email
    pub email: String,

    /// purify password without pass hashing
    pub password: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPayload {
    /// user email
    pub email: String,

    /// purify password without pass hashing
    pub password: String,
}
