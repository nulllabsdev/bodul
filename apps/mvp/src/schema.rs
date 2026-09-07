// @generated automatically by Diesel CLI.

diesel::table! {
    command_entries (id) {
        id -> Uuid,
        command_type -> Text,
        status -> Int4,
        payload -> Text,
        meta -> Nullable<Jsonb>,
        scheduled_at -> Timestamptz,
        attempts -> Int4,
        reservation_id -> Nullable<Uuid>,
        reserved_at -> Nullable<Timestamptz>,
        received_at -> Timestamptz,
        updated_at -> Timestamptz,
        processed_at -> Nullable<Timestamptz>,
        extra_info -> Nullable<Jsonb>,
    }
}

diesel::table! {
    event_entries (id) {
        id -> Uuid,
        event_type -> Text,
        status -> Int4,
        payload -> Text,
        meta -> Nullable<Jsonb>,
        scheduled_at -> Timestamptz,
        attempts -> Int4,
        reservation_id -> Nullable<Uuid>,
        reserved_at -> Nullable<Timestamptz>,
        received_at -> Timestamptz,
        updated_at -> Timestamptz,
        processed_at -> Nullable<Timestamptz>,
        extra_info -> Nullable<Jsonb>,
    }
}

diesel::table! {
    grouped_sitemap_contents (id) {
        id -> Uuid,
        processed_sitemap_id -> Uuid,
        retrieval_id -> Uuid,
        retailer_code -> Text,
        content -> Jsonb,
        product_count -> Int4,
        catalog_count -> Int4,
        content_count -> Int4,
        not_interested_count -> Int4,
        unknown_count -> Int4,
        grouped_at -> Timestamptz,
        content_size -> Int4,
    }
}

diesel::table! {
    inbox_entries (id) {
        id -> Uuid,
        status -> Int4,
        payload -> Text,
        meta -> Jsonb,
        scheduled_at -> Timestamptz,
        attempts -> Int4,
        reservation_id -> Nullable<Uuid>,
        reserved_at -> Nullable<Timestamptz>,
        received_at -> Timestamptz,
        updated_at -> Timestamptz,
        processed_at -> Nullable<Timestamptz>,
        extra_info -> Nullable<Jsonb>,
    }
}

diesel::table! {
    offers (id) {
        id -> Uuid,
        grouped_content_id -> Uuid,
        retailer_code -> Text,
        url -> Text,
        status -> Text,
        notes -> Nullable<Text>,
        discovered_at -> Timestamptz,
    }
}

diesel::table! {
    outbox_entries (id) {
        id -> Uuid,
        status -> Int4,
        payload -> Text,
        meta -> Jsonb,
        scheduled_at -> Timestamptz,
        attempts -> Int4,
        reservation_id -> Nullable<Uuid>,
        reserved_at -> Nullable<Timestamptz>,
        received_at -> Timestamptz,
        updated_at -> Timestamptz,
        processed_at -> Nullable<Timestamptz>,
        last_error -> Nullable<Text>,
        extra_info -> Nullable<Jsonb>,
    }
}

diesel::table! {
    processed_sitemaps (id) {
        id -> Uuid,
        retrieval_id -> Uuid,
        retailer_code -> Text,
        document -> Jsonb,
        url_count -> Int4,
        processed_at -> Timestamptz,
        document_size -> Int4,
    }
}

diesel::table! {
    raw_offers (id) {
        id -> Uuid,
        offer_id -> Uuid,
        url -> Text,
        body -> Text,
        body_size -> Int4,
        fetched_at -> Timestamptz,
    }
}

diesel::table! {
    raw_sitemap_documents (id) {
        id -> Uuid,
        retrieval_id -> Uuid,
        url -> Text,
        last_modified -> Nullable<Timestamptz>,
        body -> Text,
        fetched_at -> Timestamptz,
        body_size -> Int4,
    }
}

diesel::table! {
    sitemap_retrievals (id) {
        id -> Uuid,
        retailer_code -> Text,
        status -> Text,
        requested_at -> Timestamptz,
        retrieved_at -> Nullable<Timestamptz>,
        processed_at -> Nullable<Timestamptz>,
        grouped_at -> Nullable<Timestamptz>,
        error -> Nullable<Text>,
    }
}

diesel::joinable!(grouped_sitemap_contents -> processed_sitemaps (processed_sitemap_id));
diesel::joinable!(grouped_sitemap_contents -> sitemap_retrievals (retrieval_id));
diesel::joinable!(processed_sitemaps -> sitemap_retrievals (retrieval_id));
diesel::joinable!(raw_sitemap_documents -> sitemap_retrievals (retrieval_id));

diesel::allow_tables_to_appear_in_same_query!(
    command_entries,
    event_entries,
    grouped_sitemap_contents,
    inbox_entries,
    offers,
    outbox_entries,
    processed_sitemaps,
    raw_offers,
    raw_sitemap_documents,
    sitemap_retrievals,
);
