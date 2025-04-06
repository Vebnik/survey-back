use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    Extension,
};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};

use crate::{user::User, Ctx, Result};

pub async fn jwt_auth(
    Extension(ctx): Extension<Ctx>,
    Extension(user): Extension<User>,
    req: Request,
    next: Next,
) -> Result<Response> {
    // let email = user.email.clone();

    // if !ctx.config.whitelist_emails.contains(&email) {
    //     return Err(Error::Unauthorized);
    // }

    Ok(next.run(req).await)   
}