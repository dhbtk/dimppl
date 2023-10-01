// @generated automatically by Diesel CLI.

diesel::table! {
    podcast_episodes (id) {
        id -> Int8,
        podcast_id -> Int8,
        guid -> Text,
        url -> Text,
        listened_seconds -> Int4,
        completed -> Bool,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    podcasts (id) {
        id -> Int8,
        user_id -> Int8,
        guid -> Text,
        url -> Text,
        updated_at -> Timestamp,
    }
}

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

diesel::joinable!(podcast_episodes -> podcasts (podcast_id));
diesel::joinable!(podcasts -> users (user_id));
diesel::joinable!(user_devices -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(podcast_episodes, podcasts, user_devices, users,);
