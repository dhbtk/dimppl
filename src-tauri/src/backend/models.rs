use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateDeviceRequest {
    pub user_access_key: String,
    pub device_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDeviceResponse {
    pub name: String,
    pub access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub access_key: String,
}
