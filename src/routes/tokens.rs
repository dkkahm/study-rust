use actix_web::{web, HttpResponse, ResponseError};

use crate::authentication::{middleware::UserId, AuthError};

#[derive(thiserror::Error, Debug)]
pub enum TokenError {
    #[error("Authentication failed")]
    AuthError(#[source] AuthError),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl From<AuthError> for TokenError {
    fn from(error: AuthError) -> Self {
        Self::AuthError(error)
    }
}

impl ResponseError for TokenError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            TokenError::AuthError(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            TokenError::UnexpectedError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(
    name = "Test token",
    skip(user_id),
)]
pub async fn token_test(
    user_id: web::ReqData<UserId>
) -> Result<HttpResponse, TokenError> {
    // println!("### token_test >>>");
    let user_id = user_id.into_inner();
    // println!("### token_test: user_id = {:?}", user_id);
    Ok(HttpResponse::Ok().body(user_id.to_string()))
}
