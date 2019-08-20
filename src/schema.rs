table! {
    products (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        img -> Varchar,
        kind -> Varchar,
        duration -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        nickname -> Varchar,
        email -> Varchar,
        password -> Varchar,
        activated -> Bool,
        activated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    products,
    users,
);
