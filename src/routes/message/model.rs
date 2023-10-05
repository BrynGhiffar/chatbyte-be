
use anyhow::anyhow;
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::extract::Query;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::Deserialize;

use crate::routes::FailedResponse;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiverUidQuery {
    pub receiver_uid: i32
}

#[async_trait]
impl<S> FromRequestParts<S> for ReceiverUidQuery 
    where S: Send + Sync
{
    type Rejection = Response;
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S
    ) ->  Result<Self, Self::Rejection> 
    {

        let Query(res): Query<ReceiverUidQuery> = Query::try_from_uri(&parts.uri)
            .map_err(|_| FailedResponse(anyhow!("receiverUid query parameter missing")).into_response())?;
        Ok(res)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupIdQuery {
    pub group_id: i32
}

#[async_trait]
impl<S> FromRequestParts<S> for GroupIdQuery 
    where S: Send + Sync
{
    type Rejection = Response;
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S
    ) ->  Result<Self, Self::Rejection> 
    {

        let Query(res): Query<GroupIdQuery> = Query::try_from_uri(&parts.uri)
            .map_err(|_| FailedResponse(anyhow!("groupId query parameter missing")).into_response())?;
        Ok(res)
    }
}