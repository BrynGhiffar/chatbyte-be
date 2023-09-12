use std::{
    collections::BTreeMap,
    future::{ready, Ready},
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::Error;
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue},
    HttpRequest, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use serde_json::json;
use sha2::Sha256;

pub fn verify_token(token: String) -> Result<i32, String> {
    let token = token.trim_start_matches("Bearer ");
    let Some(secret) = std::env::var("JWT_SECRET").ok() else {
        return Err("Secret key to verify to jwt is missing".to_string());
    };
    let Some(key): Option<Hmac<Sha256>> = Hmac::new_from_slice(secret.as_bytes()).ok() else {
        return Err("Error creating secret key hmac".to_string());
    };

    let Some(claims): Option<BTreeMap<String, u64>> = token.verify_with_key(&key).ok() else {
        return Err("Error decoding token payload".to_string());
    };

    let Some(uid): Option<u64> = claims.get("uid").map(|n| n.clone()) else {
        return Err("Uid is missing from payload".to_string());
    };

    let Some(expiration): Option<u64> = claims.get("expiration").map(|n| n.clone()) else {
        return Err("Expiration is missing from headers".to_string());
    };

    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if expiration <= current_timestamp {
        return Err("Token has expired".to_string());
    };

    let Some(uid) = i32::try_from(uid).ok() else {
        return Err("UID cannot be cast from payload".to_string());
    };

    return Ok(uid);
}

pub fn get_uid_from_header(req: HttpRequest) -> Option<i32> {
    let uid = req
        .headers()
        .get("uid")
        .map(|v| v.to_str().ok())
        .flatten()
        .map(|s| s.to_string())
        .map(|s| s.parse::<i32>().ok())
        .flatten();
    uid
}

pub struct VerifyToken;

impl<S, B> Transform<S, ServiceRequest> for VerifyToken
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = VerifyTokenMiddleWare<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(VerifyTokenMiddleWare { service }))
    }
}

pub struct VerifyTokenMiddleWare<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for VerifyTokenMiddleWare<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let bad_request = |req: ServiceRequest, message: String| -> Self::Future {
            let (req, _pl) = req.into_parts();
            let res = HttpResponse::BadRequest()
                .json(json!({
                    "success": false,
                    "message": message
                }))
                .map_into_right_body();
            return Box::pin(async { Ok(ServiceResponse::new(req, res)) });
        };
        let headers = req.headers_mut();
        let Some(token) = headers.get("Authorization") else {
            return bad_request(req, "Token is missing".to_string());
        };
        let Ok(token) = token.to_str() else {
            return bad_request(req, "Cannot convert token to string".to_string());
        };

        let uid = match verify_token(token.to_string()) {
            Ok(uid) => uid,
            Err(msg) => return bad_request(req, msg),
        };
        let uid = uid.to_string();
        let Ok(uid) = HeaderValue::from_str(&uid) else {
            return bad_request(req, "Failed to convert id to header value".to_string());
        };
        headers.append(HeaderName::from_static("uid"), uid);

        let fut = self.service.call(req);
        Box::pin(async move { fut.await.map(ServiceResponse::map_into_left_body) })
    }
}
