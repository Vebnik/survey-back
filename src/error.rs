use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use serde_with::DisplayFromStr;
use validator::ValidationErrors;

/// An API-friendly error type.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An internal error occurred.
    #[error("An internal server error occurred")]
    InternalError,

    /// A SQLx call returned an error.
    ///
    /// The exact error contents are not reported to the user in order to avoid leaking
    /// information about databse internals.
    #[error("An internal database error occurred")]
    Sqlx(#[from] sqlx::Error),

    /// Similarly, we don't want to report random `anyhow` errors to the user.
    #[error("An internal server error occurred")]
    Anyhow(#[from] anyhow::Error),

    #[error("Validation error in request body, {0}")]
    InvalidEntity(#[from] ValidationErrors),

    #[error("uuid error")]
    Uuid(#[from] uuid::Error),

    #[error("task error: {0}")]
    TaskError(#[from] tokio::task::JoinError),

    #[error("json error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("http error: {0}")]
    HttpError(#[from] reqwest::Error),

    // UNAUTHORIZED
    #[error("Unauthorized")]
    Unauthorized,

    #[error("No cookies provided")]
    NoCookies,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,

    #[error("Token cannot be used yet")]
    TokenCannotBeUsedYet,

    // BAD_REQUEST
    #[error("Redirect not allowed")]
    RedirectNotAllowed,

    #[error("Jwk parse error")]
    JwkParse,

    // NOT_FOUND
    #[error("User not found")]
    UserNotFound,
}

#[serde_with::serde_as]
#[serde_with::skip_serializing_none]
#[derive(serde::Serialize)]
struct ErrorResponse<'a> {
    // Serialize the `Display` output as the error message
    #[serde_as(as = "DisplayFromStr")]
    message: &'a Error,
    errors: Option<&'a ValidationErrors>,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let errors = match &self {
            Error::InvalidEntity(errors) => Some(errors),
            _ => None,
        };

        // Normally we wouldn't just print this, but it's useful for debugging without
        // using a logging framework.
        println!("API error: {self:?}");

        (
            self.status_code(),
            Json(ErrorResponse {
                message: &self,
                errors,
            }),
        )
            .into_response()
    }
}

impl Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;

        match self {
            InternalError | Sqlx(_) | Anyhow(_) | TaskError(_) | HttpError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            InvalidEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            NoCookies | InvalidToken | Unauthorized | TokenExpired | TokenCannotBeUsedYet => {
                StatusCode::UNAUTHORIZED
            }
            RedirectNotAllowed | JwkParse | Uuid(_) | SerdeJson(_) => StatusCode::BAD_REQUEST,
            UserNotFound => StatusCode::NOT_FOUND,
        }
    }
}

/// A convenience type alias for `Result<T, Error>`.
pub type Result<T, E = Error> = anyhow::Result<T, E>;
