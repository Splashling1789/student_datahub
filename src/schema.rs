// @generated automatically by Diesel CLI.

diesel::table! {
    entry (id) {
        id -> Integer,
        date -> Date,
        subject_id -> Integer,
        dedicated_time -> Integer,
    }
}

diesel::table! {
    periods (id) {
        id -> Integer,
        initial_date -> Date,
        final_date -> Date,
        description -> Text,
    }
}

diesel::table! {
    subjects (id) {
        id -> Integer,
        period_id -> Integer,
        short_name -> Text,
        name -> Text,
        final_score -> Nullable<Float>,
    }
}

diesel::joinable!(entry -> subjects (subject_id));
diesel::joinable!(subjects -> periods (period_id));

diesel::allow_tables_to_appear_in_same_query!(
    entry,
    periods,
    subjects,
);
