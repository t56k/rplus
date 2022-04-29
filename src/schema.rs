table! {
    brands (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Nullable<Varchar>,
        canvas -> Nullable<Bool>,
        categories -> Nullable<Bool>,
        charged -> Nullable<Bool>,
        custom_articles -> Nullable<Bool>,
        editable -> Nullable<Bool>,
        frames -> Nullable<Bool>,
        mailchimp -> Nullable<Bool>,
        pdfs -> Nullable<Bool>,
        photos -> Nullable<Bool>,
        posting -> Nullable<Bool>,
        powerpoint -> Nullable<Bool>,
        premium -> Nullable<Bool>,
        schedulable -> Nullable<Bool>,
        trialable -> Nullable<Bool>,
        word -> Nullable<Bool>,
        wordpress -> Nullable<Bool>,
    }
}

table! {
    documents (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        title -> Varchar,
        body -> Text,
        status -> Varchar,
        issue -> Varchar,
        user_id -> Int4,
        custom -> Nullable<Bool>,
        edited -> Nullable<Bool>,
        ordered -> Nullable<Bool>,
        outdated -> Nullable<Bool>,
        premium -> Nullable<Bool>,
        priced -> Nullable<Bool>,
        retired -> Nullable<Bool>,
    }
}

table! {
    users (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Varchar,
        email -> Varchar,
        encrypted_password -> Varchar,
        status -> Varchar,
        balance -> Nullable<Money>,
        brand_id -> Int4,
        client_code -> Varchar,
        colour1_cmyk -> Array<Int4>,
        colour2_cmyk -> Array<Int4>,
        colour1_hex -> Varchar,
        colour2_hex -> Varchar,
        details -> Nullable<Array<Jsonb>>,
        document_count -> Nullable<Int4>,
        latitude -> Nullable<Float8>,
        longitude -> Nullable<Float8>,
        disclosure -> Nullable<Text>,
        general_advice -> Nullable<Text>,
        map_address -> Nullable<Varchar>,
    }
}

joinable!(documents -> users (user_id));
joinable!(users -> brands (brand_id));

allow_tables_to_appear_in_same_query!(brands, documents, users,);
