// @generated automatically by Diesel CLI.

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

diesel::allow_tables_to_appear_in_same_query!(
    locations,
    users,
);
