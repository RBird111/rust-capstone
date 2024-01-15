// @generated automatically by Diesel CLI.

diesel::table! {
    businesses (id) {
        id -> Int4,
        #[max_length = 40]
        name -> Varchar,
        description -> Text,
        #[max_length = 40]
        category -> Varchar,
        location_id -> Int4,
        owner_id -> Nullable<Int4>,
    }
}

diesel::table! {
    locations (id) {
        id -> Int4,
        #[max_length = 40]
        address -> Varchar,
        #[max_length = 40]
        city -> Varchar,
        #[max_length = 40]
        state -> Varchar,
        lat -> Nullable<Numeric>,
        lng -> Nullable<Numeric>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 40]
        first_name -> Varchar,
        #[max_length = 40]
        last_name -> Varchar,
        #[max_length = 40]
        username -> Varchar,
        #[max_length = 40]
        email -> Varchar,
        hashed_password -> Text,
    }
}

diesel::joinable!(businesses -> locations (location_id));
diesel::joinable!(businesses -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
    businesses,
    locations,
    users,
);
