use std::sync::OnceLock;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request, StatusCode},
    response::IntoResponse,
    routing, Json, RequestPartsExt, Router,
};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) sub: String,
    pub(crate) exp: u64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct LoginRequest {
    pub(crate) username: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct LoginResponse {
    pub(crate) token: String,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredential,
    #[error("Failed to generate JWT token")]
    TokenGenerationError,
}

pub(crate) async fn login_handler(
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AuthError> {
    if payload.username != "meteor" {
        return Err(AuthError::InvalidCredential);
    }

    let claim = Claims {
        sub: payload.username,
        exp: (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as u64,
    };

    Ok(Json(LoginResponse {
        token: jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claim,
            &JwtKeys::get().encoding_key,
        )
        .map_err(|_| AuthError::TokenGenerationError)?,
    }))
}

pub(crate) fn routes() -> Router {
    Router::new().route("/login", routing::post(login_handler))
}

// FIXME: This secret need to be fix.
const SECRET: &[u8] = b"secret";
static KEYS: OnceLock<JwtKeys> = OnceLock::new();

struct JwtKeys {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtKeys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }

    fn get() -> &'static Self {
        KEYS.get_or_init(|| Self::new(SECRET))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidCredential)?;

        let token_data = jsonwebtoken::decode::<Claims>(
            bearer.token(),
            &JwtKeys::get().decoding_key,
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|_| AuthError::InvalidCredential)?;

        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AuthError::InvalidCredential => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AuthError::TokenGenerationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to generate JWT token",
            ),
        }
        .into_response()
    }
}

#[cfg(test)]
mod tests {
    use axum::http::Request;
    use googletest::prelude::*;

    use super::*;

    #[googletest::test]
    #[tokio::test]
    async fn when_cred_correct_returns_validate_jwt() -> Result<()> {
        let Json(LoginResponse { token }) = login_handler(Json(LoginRequest {
            username: "meteor".to_string(),
        }))
        .await?;
        let (mut request_part, _) = Request::get("http://localhost/")
            .header("Authorization", format!("Bearer {}", token))
            .body(())?
            .into_parts();

        verify_that!(
            Claims::from_request_parts(&mut request_part, &()).await,
            ok(pat!(Claims { sub: eq("meteor") }))
        )
    }

    #[googletest::test]
    #[tokio::test]
    async fn when_cred_incorrect_auth_returns_error() -> Result<()> {
        verify_that!(
            login_handler(Json(LoginRequest {
                username: "meteor2".to_string(),
            }))
            .await,
            err(pat!(AuthError::InvalidCredential))
        )
    }

    #[googletest::test]
    #[tokio::test]
    async fn when_token_incorrect_validate_returns_error() -> Result<()> {
        let (mut request_part, _) = Request::get("http://localhost/")
            .header("Authorization", "Bearer invalid-token")
            .body(())?
            .into_parts();

        verify_that!(
            Claims::from_request_parts(&mut request_part, &()).await,
            err(pat!(AuthError::InvalidCredential))
        )
    }
}
