// @generated automatically by Diesel CLI.

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
