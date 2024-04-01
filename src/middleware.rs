use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};

use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
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
