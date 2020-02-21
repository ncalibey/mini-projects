table! {
    comments (id) {
        id -> Integer,
        user_id -> Integer,
        post_id -> Integer,
        body -> Text,
    }
}

table! {
    posts (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
    }
}

joinable!(comments -> posts (post_id));
joinable!(comments -> users (user_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    users,
);
