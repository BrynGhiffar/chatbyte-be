use std::collections::BTreeMap;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use anyhow::anyhow;
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::extract::Query;
use axum::http::request::Parts;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::VerifyWithKey;
use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::Serialize;
use sha2::Sha256;
use thiserror::Error;

use crate::app::AppState;
use crate::repository::AttachmentFileType;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiverUidQuery {
    pub receiver_uid: i32,
}

#[async_trait]
impl<S> FromRequestParts<S> for ReceiverUidQuery
where
    S: Send + Sync,
{
    type Rejection = Response;
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Query(res): Query<ReceiverUidQuery> =
            Query::try_from_uri(&parts.uri).map_err(|_| {
                FailedResponse(anyhow!("receiverUid query parameter missing")).into_response()
            })?;
        Ok(res)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupIdQuery {
    pub group_id: i32,
}

#[async_trait]
impl<S> FromRequestParts<S> for GroupIdQuery
where
    S: Send + Sync,
{
    type Rejection = Response;
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Query(res): Query<GroupIdQuery> = Query::try_from_uri(&parts.uri).map_err(|_| {
            FailedResponse(anyhow!("groupId query parameter missing")).into_response()
        })?;
        Ok(res)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenQuery {
    pub token: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for TokenQuery
where
    S: Send + Sync,
{
    type Rejection = Response;
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Query(res): Query<TokenQuery> = Query::try_from_uri(&parts.uri).map_err(|_| {
            FailedResponse(anyhow!("token query parameter missing")).into_response()
        })?;
        Ok(res)
    }
}

pub struct AuthorizedUserFromTokenQuery {
    pub user_id: i32,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthorizedUserFromTokenQuery {
    type Rejection = Response;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TokenQuery { token } = TokenQuery::from_request_parts(parts, state).await?;
        let secret = &state.env_jwt_secret;
        let AuthorizedUser { user_id } =
            AuthorizedUser::authenticate(&token, secret).map_err(|e| e.into_response())?;
        Ok(AuthorizedUserFromTokenQuery { user_id })
    }
}

pub struct AttachmentResponse(pub Vec<u8>, pub AttachmentFileType);

impl IntoResponse for AttachmentResponse {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::OK;
        let mut response = (status, self.0).into_response();
        let headers = response.headers_mut();
        headers.remove("Content-Type");
        match self.1 {
            AttachmentFileType::Png => {
                headers.append("Content-Type", HeaderValue::from_static("image/png"));
            }
            AttachmentFileType::Jpeg => {
                headers.append("Content-Type", HeaderValue::from_static("image/jpeg"));
            }
        };
        return response;
    }
}

pub struct ImageResponse(pub Vec<u8>);

impl IntoResponse for ImageResponse {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::OK;
        let mut response = (status, self.0).into_response();
        let headers = response.headers_mut();
        headers.remove("Content-Type");
        headers.append("Content-Type", HeaderValue::from_static("image/png"));
        return response;
    }
}

pub enum ServerResponse<T>
where
    T: Serialize,
{
    Success(T),
    Failed(anyhow::Error),
}

impl<E, T> From<E> for ServerResponse<T>
where
    T: Serialize,
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self::Failed(value.into())
    }
}

impl<T> IntoResponse for ServerResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            Self::Success(_) => StatusCode::OK,
            Self::Failed(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(self)).into_response()
    }
}

impl<T> Serialize for ServerResponse<T>
where
    T: Serialize,
{
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Success(payload) => {
                let mut state = serializer.serialize_struct("Success", 2)?;
                state.serialize_field("success", &true)?;
                state.serialize_field("payload", payload)?;
                state.end()
            }
            Self::Failed(err) => {
                let mut state = serializer.serialize_struct("Failed", 2)?;
                state.serialize_field("success", &false)?;
                state.serialize_field("message", &err.to_string())?;
                state.end()
            }
        }
    }
}

pub struct FailedResponse(pub anyhow::Error);

impl Serialize for FailedResponse {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("FailedResponse", 2)?;
        state.serialize_field("success", &false)?;
        state.serialize_field("message", &self.0.to_string())?;
        state.end()
    }
}

impl IntoResponse for FailedResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

pub struct AuthorizedUser {
    pub user_id: i32,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthorizedUser
// where S: Send + Sync
{
    type Rejection = TokenAuthenticationError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get("Authorization");
        let auth_header = match auth_header {
            Some(value) => value,
            None => return Err(TokenAuthenticationError::TokenIsMissing),
        };
        let auth_token = match auth_header.to_str() {
            Ok(value) => value,
            Err(_) => return Err(TokenAuthenticationError::InvalidToken),
        };
        let auth_token = auth_token.trim_start_matches("Bearer ");
        Self::authenticate(&auth_token, &state.env_jwt_secret)
    }
}

impl AuthorizedUser {
    fn authenticate(
        token: &str,
        secret: &str,
    ) -> Result<AuthorizedUser, TokenAuthenticationError> {
        let key: Hmac<Sha256> = match Hmac::new_from_slice(secret.as_bytes()) {
            Ok(key) => key,
            Err(e) => return Err(TokenAuthenticationError::Other(e.into())),
        };
        let claims: BTreeMap<String, u64> = match token.verify_with_key(&key) {
            Ok(claims) => claims,
            Err(_) => return Err(TokenAuthenticationError::InvalidToken),
        };
        let (user_id, expiration) = match (claims.get("uid"), claims.get("expiration")) {
            (Some(uid), Some(expiration)) => (uid, expiration),
            _ => return Err(TokenAuthenticationError::InvalidToken),
        };
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if *expiration < now {
            return Err(TokenAuthenticationError::TokenExpired);
        }
        let user_id = match i32::try_from(*user_id) {
            Ok(v) => v,
            Err(_) => return Err(TokenAuthenticationError::InvalidToken),
        };

        Ok(AuthorizedUser { user_id })
    }
}

#[derive(Error, Debug)]
pub enum TokenAuthenticationError {
    #[error("Token is invalid")]
    InvalidToken,
    #[error("Token has expired")]
    TokenExpired,
    #[error("Token is missing")]
    TokenIsMissing,
    #[error("{0}")]
    Other(anyhow::Error),
}

impl IntoResponse for TokenAuthenticationError {
    fn into_response(self) -> axum::response::Response {
        FailedResponse(self.into()).into_response()
    }
}
