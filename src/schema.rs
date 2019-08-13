table! {
    users (id) {
        id -> Int4,
        token -> Nullable<Varchar>,
        username -> Varchar,
        nickname -> Varchar,
        email -> Varchar,
        password -> Varchar,
        activated -> Bool,
        activated_at -> Nullable<Timestamp>,
        expired_at -> Timestamp,
    }
}
