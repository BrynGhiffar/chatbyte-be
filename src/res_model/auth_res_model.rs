use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoginResponse {
    Success {
        success: bool,
        payload: String
    },
    Failed {
        success: bool,
        message: String
    }
}

impl LoginResponse {
    pub fn failed(message: String) -> Self {
        return Self::Failed { success: false, message: message }
    }
    pub fn success(payload: String) -> Self {
        return Self::Success { success: true, payload: payload }
    }
}