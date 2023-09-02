use crate::backend::models::{CreateDeviceRequest, CreateDeviceResponse, CreateUserResponse};
use crate::errors::AppResult;
use crate::environment::API_URL;

pub async fn create_user() -> AppResult<CreateUserResponse> {
    let client = reqwest::Client::new();
    let response = client.post(format!("{API_URL}/user"))
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
