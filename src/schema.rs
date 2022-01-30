table! {
    favorites (id) {
        id -> Text,
        pattern_id -> Text,
        user_id -> Text,
    }
}

table! {
    patterns (id) {
        id -> Text,
        user_id -> Text,
        name -> Text,
        homepage_url -> Text,
        highlight_image_url -> Nullable<Text>,
        created_at -> Timestamp,
        times_visited_in_24_hours -> Integer,
    }
}

table! {
    users (id) {
        id -> Text,
        first_name -> Text,
        last_name -> Text,
        username -> Text,
        password -> Text,
    }
}

joinable!(favorites -> patterns (pattern_id));
joinable!(favorites -> users (user_id));
joinable!(patterns -> users (user_id));

allow_tables_to_appear_in_same_query!(
    favorites,
    patterns,
    users,
);
