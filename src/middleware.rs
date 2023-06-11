use std::{collections::BTreeMap, future::{Ready, ready}};

use actix_web::{dev::{Transform, ServiceRequest, Service, ServiceResponse, self}, HttpResponse, body::EitherBody, http::header::{HeaderValue, HeaderName}};
use actix_web::Error;
use futures_util::future::LocalBoxFuture;
use hmac::{ Hmac, Mac };
use jwt::VerifyWithKey;
use serde_json::json;
use sha2::Sha256;


pub fn verify_token(token: String) -> Option<u32> {
    let token = token.trim_start_matches("Bearer ");
    let Some(secret) = std::env::var("JWT_SECRET").ok() else {
        log::info!("JWT SECRET IS MISSING");
        return None;
    };
    let Some(key): Option<Hmac<Sha256>> = Hmac::new_from_slice(secret.as_bytes()).ok() else {
        log::info!("Error creating key");
        return None;
    };
    
    let Some(claims): Option<BTreeMap<String, u32>> = token.verify_with_key(&key).ok() else {
        log::info!("Error creating claims");
        return None;
    };
    return claims.get("uid").map(|n| n.clone());
}

pub struct VerifyToken;

impl<S, B> Transform<S, ServiceRequest> for VerifyToken 
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
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
    service: S
}

impl<S, B> Service<ServiceRequest> for VerifyTokenMiddleWare<S> 
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let bad_request = |req: ServiceRequest, message: String| -> Self::Future {
            let (req, _pl) = req.into_parts();
            let res = HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": message
            })).map_into_right_body();
            return Box::pin(async { Ok(ServiceResponse::new(req, res))});
        };
        // println!("Called");
        let headers = req.headers_mut();
        let Some(token) = headers.get("Authorization") else {
            return bad_request(req, "Token is missing".to_string());
        };
        let Ok(token) = token.to_str() else {
            return bad_request(req, "Cannot convert token to string".to_string());
        };
        let Some(uid) = verify_token(token.to_string()) else {
            return bad_request(req, "Token is invalid".to_string());
        };
        let uid = uid.to_string();
        let Ok(uid) = HeaderValue::from_str(&uid) else {
            return bad_request(req, "Failed to convert id to header value".to_string());
        };
        headers.append(HeaderName::from_static("uid"), uid);

        let fut = self.service.call(req);
        Box::pin(async move {
            fut.await.map(ServiceResponse::map_into_left_body)
        })
    }

}