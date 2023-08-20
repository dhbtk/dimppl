use axum::extract::State;
use axum::Json;
use deadpool_diesel::postgres::Pool;
use diesel::{Insertable, RunQueryDsl, SelectableHelper};
use diesel::associations::HasTable;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::models::User;
use crate::schema::users::dsl::users;

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub access_key: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
struct NewUser {
    access_key: String,
}

impl From<User> for CreateUserResponse {
    fn from(value: User) -> Self {
        Self {
            access_key: value.access_key
        }
    }
}

impl NewUser {
    fn new() -> Self {
        Self {
            access_key: generate_access_key()
        }
    }
}

fn random_chars(count: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(count)
        .map(char::from)
        .collect::<String>()
        .to_uppercase()
}

fn generate_access_key() -> String {
    format!(
        "{}-{}-{}-{}-{}",
        random_chars(8),
        random_chars(4),
        random_chars(4),
        random_chars(10),
        random_chars(2)
    )
}

pub async fn create_user(State(pool): State<Pool>) -> Json<CreateUserResponse> {
    let conn = pool.get().await.unwrap();
    loop {
        let result = conn.interact(|conn| {
            let new_user = NewUser::new();
            let query = diesel::insert_into(users::table())
                .values(&new_user)
                .returning(User::as_returning());
            query.get_result(conn)
        }).await.unwrap();
        if let Ok(user) = result {
            return Json(user.into());
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    // for `call`
    use tower::ServiceExt;

    use crate::app::create_test_app;
    use serial_test::serial;

    use super::*;

    #[tokio::test]
    #[serial]
    async fn test_create_user_happy_path() {
        let app = create_test_app();

        let response = app
            .oneshot(Request::builder().method("POST").uri("/user").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: CreateUserResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(32, body.access_key.len());
    }
}
