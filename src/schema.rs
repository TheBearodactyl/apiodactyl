// @generated automatically by Diesel CLI.

diesel::table! {
    api_keys (id) {
        id -> Int4,
        key_hash -> Varchar,
        is_admin -> Bool,
        created_at -> Timestamp,
        last_used_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Text,
        author -> Text,
        genres -> Array<Nullable<Text>>,
        tags -> Array<Nullable<Text>>,
        rating -> Int4,
        status -> Text,
        description -> Text,
        my_thoughts -> Text,
        links -> Nullable<Array<Nullable<Jsonb>>>,
        cover_image -> Text,
        explicit -> Bool,
        color -> Nullable<Text>,
    }
}

diesel::table! {
    games (id) {
        id -> Int4,
        title -> Text,
        developer -> Text,
        genres -> Array<Nullable<Text>>,
        tags -> Array<Nullable<Text>>,
        rating -> Int4,
        status -> Text,
        description -> Text,
        my_thoughts -> Text,
        links -> Nullable<Jsonb>,
        cover_image -> Text,
        explicit -> Bool,
        percent -> Int4,
        bad -> Bool,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        tags -> Nullable<Array<Nullable<Text>>>,
        source -> Text,
        cover_image -> Nullable<Text>,
        install_command -> Nullable<Text>,
    }
}

diesel::table! {
    reviews (id) {
        id -> Int4,
        chapter -> Int4,
        description -> Text,
        rating -> Int4,
        thoughts -> Text,
    }
}

diesel::table! {
    wplace (id) {
        id -> Int4,
        alt -> Text,
        coverimage -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    api_keys,
    books,
    games,
    projects,
    reviews,
    wplace,
);
