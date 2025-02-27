use axum::{
    extract::Request, http::header::AUTHORIZATION, middleware::Next,
    response::Response,
};

use crate::{
    app::service::jwt::{Claims, TokenType},
    library::error::{AppError::AuthError, AppResult, AuthInnerError},
};

pub async fn handle(request: Request, next: Next) -> AppResult<Response> {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| auth_value.strip_prefix("Bearer "))
        .ok_or(AuthError(AuthInnerError::InvalidToken))?;

    Claims::parse_token(token, TokenType::ACCESS, false)?;

    Ok(next.run(request).await)
}
