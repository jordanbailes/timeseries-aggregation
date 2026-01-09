// @generated automatically by Diesel CLI.

diesel::table! {
    query_history (id) {
        id -> Int4,
        query_time -> Timestamp,
        from_date -> Nullable<Date>,
        to_date -> Nullable<Date>,
        granularity -> Nullable<Text>,
    }
}

diesel::table! {
    timeseries (time) {
        time -> Timestamptz,
        quantity_kwh -> Float8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(query_history, timeseries,);
