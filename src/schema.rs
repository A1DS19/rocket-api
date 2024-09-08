// @generated automatically by Diesel CLI.

diesel::table! {
    rustacean (id) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
        created_at -> Nullable<Timestamp>,
    }
}
