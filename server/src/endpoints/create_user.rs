use axum::extract::State;
use axum::Json;

use crate::database::Pool;
use crate::error_handling::AppResult;
use serde::{Deserialize, Serialize};

use crate::models::user::NewUser;
use crate::models::{user, User};

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub access_key: String,
}

impl From<User> for CreateUserResponse {
    fn from(value: User) -> Self {
        Self {
            access_key: value.access_key,
        }
    }
}

pub async fn create_user(State(pool): State<Pool>) -> AppResult<Json<CreateUserResponse>> {
    let mut conn = pool.get().await?;
    let new_user = NewUser::default();
    let user = user::create(&new_user, &mut conn).await?;
    Ok(Json(user.into()))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::app::create_test_app;
    use serial_test::serial;

    use super::*;

    #[tokio::test]
    #[serial]
    async fn test_create_user_happy_path() {
        let (_, app) = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: CreateUserResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(32, body.access_key.len());
    }
}
