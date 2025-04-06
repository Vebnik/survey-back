use axum::{
    extract,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde_json::json;

use crate::{
    auth::MyClaims,
    user::{User, UserPayload},
    Ctx, Error, Result,
};

pub async fn auth(
    Extension(ctx): Extension<Ctx>,
    extract::Json(payload): extract::Json<UserPayload>,
) -> Result<Response> {
    let user = User::get_by_email(ctx.db.clone(), &payload.email)
        .await?
        .ok_or(Error::UserNotFound)?;

    if user.password.ne(&payload.password) {
        return Err(Error::UserNotFound);
    }

    let key: EncodingKey = EncodingKey::from_rsa_pem(ctx.config.secrets.jwt_private_key.as_ref()).unwrap();
    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some("test".to_string());

    let exp = Utc::now() + Duration::hours(1);
    let claims = MyClaims {
        id: user.id.clone(),
        iat: 1234567890,
        aud: "https://example.com".to_string(),
        exp: exp.timestamp() as u64,
    };

    let token = encode::<MyClaims>(&header, &claims, &key).unwrap();

    Ok((
        StatusCode::OK,
        Json(json!({
            "token": token,
            "user": user,
        })),
    )
        .into_response())
}
