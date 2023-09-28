use actix_multipart_extract::{File, Multipart, MultipartForm};
use actix_web::web::{self, ServiceConfig};
use actix_web::{HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::middleware::{get_uid_from_header, VerifyToken};
use crate::repository::group::{Group, GroupConversation};
use crate::utility::ApiError::*;
use crate::utility::ApiSuccess::*;
use crate::{app::AppState, utility::ApiResult};

pub fn group_config(cfg: &mut ServiceConfig) {
    cfg.route("", web::get().to(get_user_group).wrap(VerifyToken));
    cfg.route(
        "/recent",
        web::get().to(get_user_group_recent).wrap(VerifyToken),
    );
    cfg.route(
        "/message/{id}",
        web::get().to(get_group_messages).wrap(VerifyToken),
    );
    cfg.route(
        "/read/{group_id}",
        web::put().to(read_all_message).wrap(VerifyToken),
    );
    cfg.route("", web::post().to(create_group).wrap(VerifyToken));
    cfg.route("/image/{id}", web::get().to(get_group_profile_image));
}

async fn get_user_group(state: web::Data<AppState>, req: HttpRequest) -> ApiResult<Vec<Group>> {
    let uid = get_uid_from_header(req).unwrap();
    let result = state.group_repository.find_groups_for_user(uid).await;
    match result {
        Ok(groups) => Ok(Success(groups)),
        Err(e) => Err(ServerError(e)),
    }
}

async fn get_user_group_recent(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> ApiResult<Vec<GroupConversation>> {
    let user_id = get_uid_from_header(req).unwrap();
    let result = state.group_repository.find_user_group_recent(user_id).await;
    match result {
        Ok(convs) => Ok(Success(convs)),
        Err(e) => Err(ServerError(e)),
    }
}

async fn read_all_message(
    state: web::Data<AppState>,
    group_id: web::Path<(i32,)>,
    req: HttpRequest,
) -> ApiResult<&'static str> {
    let user_id = get_uid_from_header(req).unwrap();
    let (group_id,) = group_id.into_inner();
    let result = state
        .group_repository
        .read_all_message(user_id, group_id)
        .await;
    match result {
        Ok(succ) if succ => Ok(Success("Message read successfully")),
        Ok(_) => Err(ServerError("Messages were not read".to_string())),
        Err(e) => Err(ServerError(e)),
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMessageResponse {
    pub id: i32,
    pub sender_id: i32,
    pub username: String,
    pub group_id: i32,
    pub content: String,
    pub sent_at: String,
    pub deleted: bool
}

async fn create_group(
    state: web::Data<AppState>,
    form: Multipart<CreateGroupForm>,
    req: HttpRequest,
) -> ApiResult<String> {
    let uid = get_uid_from_header(req).unwrap();
    let form = form;

    let image = &form.profile_picture;
    let name = form.group_name.clone();
    let mut members = Vec::<i32>::new();
    for part in form.members.split(",") {
        let Some(id) = part.trim().parse::<i32>().ok() else {
            return Err(BadRequest("Failed parsing members list".to_string()));
        };
        members.push(id);
    }
    if !members.contains(&uid) {
        members.push(uid);
    }
    log::info!("Creating group");
    let result = state.group_repository.create_group(name).await;
    let group = match result {
        Ok(g) => g,
        Err(e) => return Err(ServerError(e)),
    };
    log::info!("adding users to group");
    for uid in members.iter() {
        let result = state
            .group_repository
            .add_user_to_group(*uid, group.id)
            .await;
        match result {
            Ok(s) if !s => return Err(ServerError("Failed adding user".to_string())),
            Ok(_) => continue,
            Err(e) => return Err(ServerError(e)),
        };
    }

    // setting profile picture for group.
    log::info!("Setting profile picture of group");
    if let Some(image) = image {
        let result = state
            .group_repository
            .set_profile_image_for_group(group.id, image.bytes.clone())
            .await;
        match result {
            Ok(s) if !s => {
                return Err(ServerError(
                    "Failed setting profile picture for group".to_string(),
                ))
            }
            Ok(_) => return Ok(Success("Successfully created group".to_string())),
            Err(e) => return Err(ServerError(e)),
        };
    }
    return Ok(Success("Successfully created group".to_string()));
}

async fn get_group_profile_image(
    state: web::Data<AppState>,
    group_id: web::Path<(i32,)>,
) -> HttpResponse {
    let empty_image = state.empty_profile.clone();
    let group_id = group_id.into_inner().0;
    let result = state
        .group_repository
        .get_profile_image_for_group(group_id)
        .await;
    match result {
        Ok(Some(img)) => HttpResponse::Ok().body(img.0),
        _ => HttpResponse::Ok().body(empty_image),
    }
}

async fn get_group_messages(
    state: web::Data<AppState>,
    req: HttpRequest,
    group_id: web::Path<(i32,)>,
) -> ApiResult<Vec<GroupMessageResponse>> {
    let uid = get_uid_from_header(req).unwrap();
    let group_id = group_id.into_inner().0;
    let result = state.group_repository.find_group_members(group_id).await;
    let members = match result {
        Ok(m) => m,
        Err(e) => return Err(ServerError(e)),
    };
    if !members.contains(&uid) {
        return Err(BadRequest("User is not a member".to_string()));
    }
    let result = state
        .group_repository
        .find_all_group_message(group_id)
        .await;
    match result {
        Ok(m) => Ok(Success(
            m
            .iter()
            .map(|m| {
                let content = if m.deleted { 
                    String::from("") 
                } else { 
                    m.content.clone() 
                };
                GroupMessageResponse {
                    id: m.id,
                    group_id: m.group_id,
                    content,
                    username: m.username.clone(),
                    sender_id: m.sender_id,
                    sent_at: m.sent_at.format("%H:%M").to_string(),
                    deleted: m.deleted
                }
            })
            .collect(),
        )),
        Err(e) => Err(ServerError(e)),
    }
}

#[derive(Deserialize, MultipartForm)]
#[serde(rename_all = "camelCase")]
struct CreateGroupForm {
    profile_picture: Option<File>,
    group_name: String,
    members: String,
}
