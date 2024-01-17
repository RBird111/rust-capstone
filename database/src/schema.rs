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
    reviews (id) {
        id -> Int4,
        rating -> Int4,
        body -> Text,
        user_id -> Int4,
        business_id -> Int4,
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

diesel::table! {
    users_locations (user_id, location_id) {
        user_id -> Int4,
        location_id -> Int4,
    }
}

diesel::joinable!(businesses -> locations (location_id));
diesel::joinable!(businesses -> users (owner_id));
diesel::joinable!(reviews -> businesses (business_id));
diesel::joinable!(reviews -> users (user_id));
diesel::joinable!(users_locations -> locations (location_id));
diesel::joinable!(users_locations -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    businesses,
    locations,
    reviews,
    users,
    users_locations,
);
