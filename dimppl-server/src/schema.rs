// @generated automatically by Diesel CLI.

diesel::table! {
    user_devices (id) {
        id -> Int8,
        user_id -> Int8,
        name -> Text,
        last_session_at -> Timestamp,
        access_token -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        access_key -> Text,
    }
}

diesel::joinable!(user_devices -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(user_devices, users,);
