// @generated automatically by Diesel CLI.

diesel::table! {
    contract (id) {
        id -> Int4,
        number -> Text,
        date_from -> Nullable<Timestamp>,
        organization_id -> Int4,
        type_of_validity -> Nullable<Int4>,
        responsible_person_id -> Nullable<Int4>,
        address -> Nullable<Text>,
        additional_agreement -> Nullable<Text>,
        comment -> Nullable<Text>,
        date_to -> Nullable<Timestamptz>,
        contract_period -> Nullable<Int4>,
        created_time -> Nullable<Timestamptz>,
        updated_time -> Nullable<Timestamptz>,
        actual -> Nullable<Bool>,
    }
}

diesel::table! {
    dict_type_of_validity (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    organization (id) {
        id -> Int4,
        short_name_with_opf -> Text,
        inn -> Int8,
        fact_address -> Nullable<Text>,
        legal_address -> Nullable<Text>,
        #[max_length = 256]
        management_post -> Nullable<Varchar>,
        #[max_length = 256]
        management_name -> Nullable<Varchar>,
        #[max_length = 256]
        ogrn -> Nullable<Varchar>,
        #[max_length = 256]
        full_name_with_opf -> Nullable<Varchar>,
        #[max_length = 256]
        opf_full -> Nullable<Varchar>,
        #[max_length = 256]
        opf_short -> Nullable<Varchar>,
    }
}

diesel::table! {
    responsible_person (id) {
        id -> Int4,
        firstname -> Text,
        name -> Text,
        lastname -> Nullable<Text>,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        login -> Varchar,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 20]
        role -> Varchar,
        is_active -> Nullable<Bool>,
    }
}

diesel::joinable!(contract -> dict_type_of_validity (type_of_validity));
diesel::joinable!(contract -> organization (organization_id));
diesel::joinable!(contract -> responsible_person (responsible_person_id));
diesel::joinable!(responsible_person -> users (user_id));

diesel::table! {
    contract_files (id) {
        id -> Int4,
        contract_fk -> Int4,
        file_name -> Text,
        orig_name -> Text,
        size_bytes -> Int8,
        mime_type_txt -> Text,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    contract,
    contract_files,
    dict_type_of_validity,
    organization,
    responsible_person,
    users,
);
