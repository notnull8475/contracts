// @generated automatically by Diesel CLI.

diesel::table! {
    contract (id) {
        id -> Int4,
        number -> Nullable<Text>,
        date -> Nullable<Timestamp>,
        organization_id -> Nullable<Int4>,
        type_of_validity -> Nullable<Int4>,
        responsible_person_id -> Nullable<Int4>,
        address -> Nullable<Text>,
        additional_agreement -> Nullable<Text>,
        comment -> Nullable<Text>,
    }
}

diesel::table! {
    dict_type_of_validity (id) {
        id -> Int4,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    organization (id) {
        id -> Int4,
        name -> Nullable<Text>,
        inn -> Nullable<Int8>,
        fact_address -> Nullable<Text>,
        address -> Nullable<Text>,
    }
}

diesel::table! {
    responsible_person (id) {
        id -> Int4,
        firstname -> Nullable<Text>,
        name -> Nullable<Text>,
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
        created_at -> Nullable<Timestamp>,
        is_active -> Nullable<Bool>,
    }
}

diesel::joinable!(contract -> dict_type_of_validity (type_of_validity));
diesel::joinable!(contract -> organization (organization_id));
diesel::joinable!(contract -> responsible_person (responsible_person_id));
diesel::joinable!(responsible_person -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    contract,
    dict_type_of_validity,
    organization,
    responsible_person,
    users,
);
