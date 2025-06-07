use anyhow::anyhow;
use axum::async_trait;
use axum::body::Bytes;
use axum::body::HttpBody;
use axum::extract::FromRequest;
use axum::extract::Multipart;
use axum::http::Request;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::BoxError;
use serde::Serialize;

use crate::routes::FailedResponse;

#[derive(Serialize)]
pub struct GroupModel {
    pub id: i32,
    pub name: String,
}

#[derive(Debug)]
pub struct CreateGroupForm {
    pub group_name: String,
    pub members: Vec<i32>,
    pub profile_picture: Option<Vec<u8>>,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for CreateGroupForm
where
    S: Send + Sync,
    B: HttpBody + Send + 'static,
    B::Data: Into<Bytes>,
    B::Error: Into<BoxError>,
{
    type Rejection = Response;

    async fn from_request(
        req: Request<B>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let res = Multipart::from_request(req, state).await;
        let mut multipart = match res {
            Ok(mp) => mp,
            Err(e) => return Err(FailedResponse(e.into()).into_response()),
        };

        let mut group_name: Option<String> = None;
        let mut group_members: Option<String> = None;
        let mut profile_picture: Option<Vec<u8>> = None;
        while let Some(field) = multipart.next_field().await.unwrap() {
            let name = field.name().unwrap().to_string();
            println!("fields: {}", name);
            if name.eq("groupName") {
                let text = match field.text().await {
                    Ok(t) => t,
                    Err(e) => return Err(FailedResponse(e.into()).into_response()),
                };
                group_name = Some(text);
            } else if name.eq("members") {
                let text = match field.text().await {
                    Ok(t) => t,
                    Err(e) => return Err(FailedResponse(e.into()).into_response()),
                };
                group_members = Some(text);
            } else if name.eq("profilePicture") {
                let data = match field.bytes().await {
                    Ok(dat) => dat,
                    Err(e) => return Err(FailedResponse(e.into()).into_response()),
                };
                let data = data.iter().cloned().collect::<Vec<_>>();
                profile_picture = Some(data);
            }
        }
        let group_name = match group_name {
            Some(s) => s,
            None => {
                return Err(FailedResponse(anyhow!("groupName field is missing")).into_response())
            }
        };
        let group_members = match group_members {
            Some(s) => s,
            None => return Err(FailedResponse(anyhow!("members field is missing")).into_response()),
        };
        let group_members = group_members
            .split(",")
            .try_fold(Vec::<i32>::new(), |mut v, it| {
                let Ok(num) = it.parse::<i32>() else {
                    return None;
                };
                v.push(num);
                Some(v)
            });
        let members = match group_members {
            Some(mm) => mm,
            None => {
                return Err(
                    FailedResponse(anyhow!("members field format is invalid")).into_response()
                )
            }
        };
        Ok(CreateGroupForm {
            group_name,
            members,
            profile_picture,
        })
    }
}
