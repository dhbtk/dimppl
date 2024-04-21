use anyhow::Context;
use axum::headers::{HeaderMap, HeaderValue};

use crate::database::AsyncConnection;
use crate::error_handling::AppResult;
use crate::models::{user, User, UserDevice};
use crate::schema::user_devices::table as user_devices;
use chrono::{NaiveDateTime, Utc};
use diesel::associations::HasTable;
use diesel::{insert_into, ExpressionMethods, Insertable, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

#[cfg(test)]
use crate::models::user::NewUser;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user_devices)]
struct NewUserDevice {
    pub user_id: i64,
    pub name: String,
    pub access_token: String,
    pub last_session_at: NaiveDateTime,
}

impl NewUserDevice {
    pub fn new(request: &CreateDeviceRequest, user: &User) -> Self {
        Self {
            user_id: user.id,
            name: request.device_name.clone(),
            access_token: generate_access_token(),
            last_session_at: Utc::now().naive_utc(),
        }
    }
}

fn generate_access_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect::<String>()
        .to_uppercase()
}

pub async fn create<'a>(
    create_request: &CreateDeviceRequest,
    user: &User,
    conn: &mut AsyncConnection<'a>,
) -> AppResult<UserDevice> {
    let user_device = insert_into(user_devices::table())
        .values(NewUserDevice::new(create_request, user))
        .returning(UserDevice::as_returning())
        .get_result(conn)
        .await?;
    Ok(user_device)
}

pub async fn user_from_http_request<'a>(
    headers: &HeaderMap<HeaderValue>,
    conn: &mut AsyncConnection<'a>,
) -> AppResult<User> {
    let (user, _) = user_and_device_from_http_request(headers, conn).await?;

    Ok(user)
}

pub async fn user_and_device_from_http_request<'a>(
    headers: &HeaderMap<HeaderValue>,
    conn: &mut AsyncConnection<'a>,
) -> AppResult<(User, UserDevice)> {
    let unauthorized = Err(crate::error_handling::AppError::unauthorized());

    let device = device_from_http_request(headers, conn).await?;
    let Ok(user) = user::find_one(device.user_id, conn).await else {
        return unauthorized;
    };

    Ok((user, device))
}

pub async fn device_from_http_request<'a>(
    headers: &HeaderMap<HeaderValue>,
    conn: &mut AsyncConnection<'a>,
) -> AppResult<UserDevice> {
    use crate::schema::user_devices::dsl::*;

    let unauthorized = Err(crate::error_handling::AppError::unauthorized());
    let Ok(token) = token_from_request(headers) else {
        return unauthorized;
    };

    let Ok(device) = user_devices
        .select(UserDevice::as_select())
        .filter(access_token.eq(token))
        .first(conn)
        .await
    else {
        return unauthorized;
    };
    Ok(device)
}

#[derive(Serialize, Deserialize)]
pub struct CreateDeviceRequest {
    pub user_access_key: String,
    pub device_name: String,
}

fn token_from_request(headers: &HeaderMap<HeaderValue>) -> AppResult<String> {
    let unauthorized = Err(crate::error_handling::AppError::unauthorized());
    let token = headers
        .get("Authorization")
        .context("bearer token not found")?
        .to_str()?;
    if let Some(token) = token.strip_prefix("Bearer ") {
        Ok(token.to_string())
    } else {
        unauthorized
    }
}

#[cfg(test)]
pub async fn test_user_and_device<'a>(
    conn: &mut AsyncConnection<'a>,
) -> AppResult<(User, UserDevice)> {
    let user = user::create(&NewUser::default(), conn).await.unwrap();
    let device = create(
        &CreateDeviceRequest {
            user_access_key: user.access_key.clone(),
            device_name: "Test Device".into(),
        },
        &user,
        conn,
    )
    .await?;
    Ok((user, device))
}
