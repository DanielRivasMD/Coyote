// @generated automatically by Diesel CLI.

diesel::table! {
    memory (id) {
        id -> Integer,
        token -> Nullable<Text>,
        kind -> Nullable<Text>,
        score -> Nullable<Integer>,
        stability -> Nullable<Integer>,
        retrievability -> Nullable<Integer>,
        difficulty -> Nullable<Integer>,
    }
}
