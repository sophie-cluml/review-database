// @generated automatically by Diesel CLI.

diesel::table! {
    category (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    cluster (id) {
        id -> Int4,
        category_id -> Int4,
        cluster_id -> Text,
        detector_id -> Int4,
        event_ids -> Array<Nullable<Int8>>,
        labels -> Nullable<Array<Nullable<Text>>>,
        last_modification_time -> Nullable<Timestamp>,
        model_id -> Int4,
        qualifier_id -> Int4,
        score -> Nullable<Float8>,
        signature -> Text,
        size -> Int8,
        status_id -> Int4,
        event_sources -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    column_description (id) {
        id -> Int4,
        column_index -> Int4,
        type_id -> Int4,
        count -> Int8,
        unique_count -> Int8,
        cluster_id -> Int4,
        batch_ts -> Timestamp,
    }
}

diesel::table! {
    csv_column_extra (id) {
        id -> Int4,
        model_id -> Int4,
        column_alias -> Nullable<Array<Nullable<Text>>>,
        column_display -> Nullable<Array<Nullable<Bool>>>,
        column_top_n -> Nullable<Array<Nullable<Bool>>>,
        column_1 -> Nullable<Array<Nullable<Bool>>>,
        column_n -> Nullable<Array<Nullable<Bool>>>,
    }
}

diesel::table! {
    csv_column_list (id) {
        id -> Int4,
        model_id -> Int4,
        column_indicator -> Nullable<Array<Nullable<Text>>>,
        column_whitelist -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    csv_indicator (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        list -> Text,
        last_modification_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    csv_whitelist (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        list -> Text,
        last_modification_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    description_binary (id) {
        id -> Int4,
        description_id -> Int4,
        mode -> Bytea,
    }
}

diesel::table! {
    description_datetime (id) {
        id -> Int4,
        description_id -> Int4,
        mode -> Timestamp,
    }
}

diesel::table! {
    description_enum (id) {
        id -> Int4,
        description_id -> Int4,
        mode -> Text,
    }
}

diesel::table! {
    description_float (id) {
        id -> Int4,
        description_id -> Int4,
        min -> Nullable<Float8>,
        max -> Nullable<Float8>,
        mean -> Nullable<Float8>,
        s_deviation -> Nullable<Float8>,
        mode_smallest -> Float8,
        mode_largest -> Float8,
    }
}

diesel::table! {
    description_int (id) {
        id -> Int4,
        description_id -> Int4,
        min -> Nullable<Int8>,
        max -> Nullable<Int8>,
        mean -> Nullable<Float8>,
        s_deviation -> Nullable<Float8>,
        mode -> Int8,
    }
}

diesel::table! {
    description_ipaddr (id) {
        id -> Int4,
        description_id -> Int4,
        mode -> Text,
    }
}

diesel::table! {
    description_text (id) {
        id -> Int4,
        description_id -> Int4,
        mode -> Text,
    }
}

diesel::table! {
    model (id) {
        id -> Int4,
        name -> Text,
        kind -> Text,
        max_event_id_num -> Int4,
        data_source_id -> Int4,
        classifier -> Bytea,
        classification_id -> Nullable<Int8>,
        version -> Int4,
    }
}

diesel::table! {
    outlier (id) {
        id -> Int4,
        raw_event -> Bytea,
        model_id -> Int4,
        event_ids -> Array<Nullable<Int8>>,
        size -> Int8,
        event_sources -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    qualifier (id) {
        id -> Int4,
        description -> Text,
    }
}

diesel::table! {
    status (id) {
        id -> Int4,
        description -> Text,
    }
}

diesel::table! {
    time_series (id) {
        id -> Int4,
        cluster_id -> Int4,
        time -> Timestamp,
        count_index -> Nullable<Int4>,
        value -> Timestamp,
        count -> Int8,
    }
}

diesel::table! {
    top_n_binary (id) {
        id -> Int4,
        description_id -> Int4,
        value -> Bytea,
        count -> Int8,
    }
}

diesel::table! {
    top_n_datetime (id) {
        id -> Int4,
        description_id -> Int4,
        value -> Timestamp,
        count -> Int8,
    }
}

diesel::table! {
    top_n_enum (id) {
        id -> Int4,
        description_id -> Int4,
        value -> Text,
        count -> Int8,
    }
}

diesel::table! {
    top_n_float (id) {
        id -> Int4,
        description_id -> Int4,
        value_smallest -> Float8,
        value_largest -> Float8,
        count -> Int8,
    }
}

diesel::table! {
    top_n_int (id) {
        id -> Int4,
        description_id -> Int4,
        value -> Int8,
        count -> Int8,
    }
}

diesel::table! {
    top_n_ipaddr (id) {
        id -> Int4,
        description_id -> Int4,
        value -> Text,
        count -> Int8,
    }
}

diesel::table! {
    top_n_text (id) {
        id -> Int4,
        description_id -> Int4,
        value -> Text,
        count -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    category,
    cluster,
    column_description,
    csv_column_extra,
    csv_column_list,
    csv_indicator,
    csv_whitelist,
    description_binary,
    description_datetime,
    description_enum,
    description_float,
    description_int,
    description_ipaddr,
    description_text,
    model,
    outlier,
    qualifier,
    status,
    time_series,
    top_n_binary,
    top_n_datetime,
    top_n_enum,
    top_n_float,
    top_n_int,
    top_n_ipaddr,
    top_n_text,
);
