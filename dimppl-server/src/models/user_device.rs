use crate::database::AsyncConnection;
use crate::error_handling::AppResult;
use crate::models::{User, UserDevice};
use crate::schema::user_devices::table as user_devices;
use chrono::{NaiveDateTime, Utc};
use diesel::associations::HasTable;
use diesel::{insert_into, Insertable, SelectableHelper};
use diesel_async::RunQueryDsl;
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

#[derive(Serialize, Deserialize)]
pub struct CreateDeviceRequest {
    pub user_access_key: String,
    pub device_name: String,
}
