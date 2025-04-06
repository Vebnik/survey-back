use uuid::Uuid;

use super::User;
use crate::{Db, Result};

impl User {
    pub async fn get_by_email(db: Db, email: &str) -> Result<Option<Self>> {
        sqlx::query_as!(User, r#"SELECT * FROM "user" WHERE "email" = $1"#, email,)
            .fetch_optional(db.as_ref())
            .await
            .map_err(Into::into)
    }

    pub async fn get_by_id(db: Db, id: Uuid) -> Result<Option<Self>> {
        sqlx::query_as!(User, r#"SELECT * FROM "user" WHERE "id" = $1"#, id,)
            .fetch_optional(db.as_ref())
            .await
            .map_err(Into::into)
    }

    pub async fn create(db: Db, email: &str, id: Uuid) -> Result<Self> {
        sqlx::query_as!(
            User,
            r#"
                INSERT INTO "user" ("id", "email") VALUES ($1, $2)
                RETURNING *
            "#,
            id,
            email,
        )
        .fetch_one(db.as_ref())
        .await
        .map_err(Into::into)
    }
}
