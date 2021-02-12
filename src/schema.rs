table! {
    sites (id) {
        id -> Int4,
        url -> Varchar,
        crawled_at -> Nullable<Timestamp>,
    }
}
