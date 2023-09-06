// @generated automatically by Diesel CLI.

diesel::table! {
    episode_progresses (id) {
        id -> Integer,
        episode_id -> Integer,
        completed -> Bool,
        listened_seconds -> Integer,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    episodes (id) {
        id -> Integer,
        guid -> Text,
        podcast_id -> Integer,
        content_local_path -> Text,
        content_url -> Text,
        description -> Text,
        image_local_path -> Text,
        image_url -> Text,
        length -> Integer,
        link -> Text,
        episode_date -> Timestamp,
        title -> Text,
    }
}

diesel::table! {
    podcasts (id) {
        id -> Integer,
        guid -> Text,
        author -> Text,
        local_image_path -> Text,
        image_url -> Text,
        feed_url -> Text,
        name -> Text,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(episode_progresses -> episodes (episode_id));
diesel::joinable!(episodes -> podcasts (podcast_id));

diesel::allow_tables_to_appear_in_same_query!(episode_progresses, episodes, podcasts,);
