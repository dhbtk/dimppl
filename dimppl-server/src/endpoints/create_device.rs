use axum::extract::State;
use axum::Json;
use diesel::Selectable;

use serde::{Deserialize, Serialize};

use crate::database::Pool;
use crate::error_handling::AppResult;
use crate::models::user_device::CreateDeviceRequest;
use crate::models::{user, user_device, UserDevice};

#[derive(Serialize, Deserialize, Selectable)]
#[diesel(table_name = crate::schema::user_devices)]
pub struct CreateDeviceResponse {
    pub name: String,
    pub access_token: String,
}

impl From<UserDevice> for CreateDeviceResponse {
    fn from(value: UserDevice) -> Self {
        Self {
            name: value.name,
            access_token: value.access_token,
        }
    }
}

pub async fn create_device(
    State(pool): State<Pool>,
    Json(create_request): Json<CreateDeviceRequest>,
) -> AppResult<Json<CreateDeviceResponse>> {
    let mut conn = pool.get().await?;
    let user = user::find_by_access_key(&create_request.user_access_key, &mut conn).await?;
    let response = user_device::create(&create_request, &user, &mut conn).await?;
    Ok(Json(response.into()))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http,
        http::{Request, StatusCode},
    };
    use serial_test::serial;
    use tower::ServiceExt;

    use crate::app::create_test_app;
    use crate::models::user::NewUser;

    use super::*;

    #[tokio::test]
    #[serial]
    async fn test_create_device_happy_path() {
        let (state, app) = create_test_app();
        let mut conn = state.pool.get().await.unwrap();
        let user = user::create(&NewUser::default(), &mut conn).await.unwrap();

        let request_body = CreateDeviceRequest {
            user_access_key: user.access_key,
            device_name: "new device".into(),
        };

        let request = Request::builder()
            .method("POST")
            .uri("/devices")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: CreateDeviceResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!("new device", body.name);
        assert_eq!(64, body.access_token.len());
    }
}
