use axum::{extract::Request, middleware::Next, response::Response, Extension};
use chrono::Utc;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::{user::User, Ctx, Error, Result};

use super::MyClaims;

pub async fn jwt_auth(
    Extension(ctx): Extension<Ctx>,
    mut req: Request,
    next: Next,
) -> Result<Response> {
    let key = DecodingKey::from_rsa_pem(ctx.config.secrets.jwt_public_key.as_ref()).unwrap();
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&["https://example.com"]);

    let token = req
        .headers()
        .get("x-token")
        .ok_or(Error::InvalidToken)?
        .to_str()
        .map_err(|_| Error::InvalidToken)?;

    let decoded = decode::<MyClaims>(token, &key, &validation).map_err(|_| Error::InvalidToken)?;

    let now_time = Utc::now().timestamp() as u64;

    if decoded.claims.exp < now_time {
        return Err(Error::TokenExpired);
    };

    let user = User::get_by_id(ctx.db.clone(), &decoded.claims.id)
        .await?
        .ok_or(Error::UserNotFound)?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
