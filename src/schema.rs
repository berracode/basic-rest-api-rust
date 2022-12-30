// @generated automatically by Diesel CLI.

diesel::table! {
    people (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        age -> Int4,
        profession -> Varchar,
        salary -> Int4,
    }
}
