use crate::backend::models::{CreateDeviceRequest, CreateDeviceResponse, CreateUserResponse};
use crate::environment::API_URL;
use crate::errors::AppResult;
use dimppl_shared::sync::{SyncStateRequest, SyncStateResponse};

pub async fn create_user() -> AppResult<CreateUserResponse> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{API_URL}/user"))
        .send()
        .await?
        .json::<CreateUserResponse>()
        .await?;
    Ok(response)
}

pub async fn create_device(request: &CreateDeviceRequest) -> AppResult<CreateDeviceResponse> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{API_URL}/devices"))
        .json(request)
        .send()
        .await?
        .json::<CreateDeviceResponse>()
        .await?;
    Ok(response)
}

pub async fn sync_remote_podcasts(token: &str, request: &SyncStateRequest) -> AppResult<SyncStateResponse> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{API_URL}/sync"))
        .header("Authorization", format!("Bearer {token}"))
        .json(request)
        .send()
        .await?
        .json::<SyncStateResponse>()
        .await?;
    Ok(response)
}
