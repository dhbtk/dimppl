use axum::extract::State;
use axum::headers::HeaderMap;
use axum::http::StatusCode;
use axum::Json;
use axum_macros::debug_handler;
use dimppl_shared::sync::{CreatePodcastEpisodeRequest, CreatePodcastRequest};

use crate::database::Pool;
use crate::error_handling::AppResult;
use crate::models::user_device;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CreatePodcastWebRequest {
    pub url: String,
    pub guid: String,
    pub episodes: Vec<CreatePodcastEpisodeWebRequest>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreatePodcastEpisodeWebRequest {
    pub url: String,
    pub guid: String,
}

impl From<CreatePodcastEpisodeWebRequest> for CreatePodcastEpisodeRequest {
    fn from(value: CreatePodcastEpisodeWebRequest) -> Self {
        Self {
            url: value.url,
            guid: value.guid,
        }
    }
}

impl CreatePodcastWebRequest {
    fn into_request(self, user_id: i64) -> CreatePodcastRequest {
        CreatePodcastRequest {
            user_id,
            url: self.url,
            guid: self.guid,
            episodes: self.episodes.into_iter().map(|e| e.into()).collect(),
        }
    }
}

#[debug_handler]
pub async fn create_podcast(
    State(pool): State<Pool>,
    headers: HeaderMap,
    Json(create_request): Json<CreatePodcastWebRequest>,
) -> AppResult<(StatusCode, ())> {
    let mut conn = pool.get().await?;
    let user = user_device::user_from_http_request(&headers, &mut conn).await?;
    let request = create_request.into_request(user.id);
    crate::models::podcast::create(&request, &mut conn).await?;
    Ok((StatusCode::CREATED, ()))
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
    use crate::models::user;
    use crate::models::user::NewUser;

    use super::*;

    #[tokio::test]
    #[serial]
    async fn test_create_podcast_happy_path() {
        let (state, app) = create_test_app();
        let mut conn = state.pool.get().await.unwrap();
        let user = user::create(&NewUser::default(), &mut conn).await.unwrap();
        let device_request = user_device::CreateDeviceRequest {
            device_name: "test".to_string(),
            user_access_key: user.access_key.clone(),
        };
        let device = user_device::create(&device_request, &user, &mut conn)
            .await
            .unwrap();

        let request_body = CreatePodcastWebRequest {
            url: "https://example.com/podcast.rss".to_string(),
            guid: "https://example.com/podcast.rss".to_string(),
            episodes: vec![
                CreatePodcastEpisodeWebRequest {
                    url: "https://example.com/episode1.mp3".to_string(),
                    guid: "https://example.com/episode1.mp3".to_string(),
                },
                CreatePodcastEpisodeWebRequest {
                    url: "https://example.com/episode2.mp3".to_string(),
                    guid: "https://example.com/episode2.mp3".to_string(),
                },
            ],
        };

        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/podcasts")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", device.access_token))
            .body(Body::from(serde_json::to_string(&request_body).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_podcast_authorization_error() {
        let (state, app) = create_test_app();
        let mut conn = state.pool.get().await.unwrap();
        let user = user::create(&NewUser::default(), &mut conn).await.unwrap();
        let device_request = user_device::CreateDeviceRequest {
            device_name: "test".to_string(),
            user_access_key: user.access_key.clone(),
        };
        let _device = user_device::create(&device_request, &user, &mut conn)
            .await
            .unwrap();

        let request_body = CreatePodcastWebRequest {
            url: "https://example.com/podcast.rss".to_string(),
            guid: "https://example.com/podcast.rss".to_string(),
            episodes: vec![
                CreatePodcastEpisodeWebRequest {
                    url: "https://example.com/episode1.mp3".to_string(),
                    guid: "https://example.com/episode1.mp3".to_string(),
                },
                CreatePodcastEpisodeWebRequest {
                    url: "https://example.com/episode2.mp3".to_string(),
                    guid: "https://example.com/episode2.mp3".to_string(),
                },
            ],
        };

        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/podcasts")
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer whatthehell")
            .body(Body::from(serde_json::to_string(&request_body).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_podcast_uniqueness() {
        let (state, app) = create_test_app();
        let mut conn = state.pool.get().await.unwrap();
        let user = user::create(&NewUser::default(), &mut conn).await.unwrap();
        let device_request = user_device::CreateDeviceRequest {
            device_name: "test".to_string(),
            user_access_key: user.access_key.clone(),
        };
        let device = user_device::create(&device_request, &user, &mut conn)
            .await
            .unwrap();

        let request_body = CreatePodcastWebRequest {
            url: "https://example.com/podcast.rss".to_string(),
            guid: "https://example.com/podcast.rss".to_string(),
            episodes: vec![
                CreatePodcastEpisodeWebRequest {
                    url: "https://example.com/episode1.mp3".to_string(),
                    guid: "https://example.com/episode1.mp3".to_string(),
                },
                CreatePodcastEpisodeWebRequest {
                    url: "https://example.com/episode2.mp3".to_string(),
                    guid: "https://example.com/episode2.mp3".to_string(),
                },
            ],
        };
        let _ =
            crate::models::podcast::create(&request_body.clone().into_request(user.id), &mut conn)
                .await;

        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/podcasts")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", device.access_token))
            .body(Body::from(serde_json::to_string(&request_body).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
