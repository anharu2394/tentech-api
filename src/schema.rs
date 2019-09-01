table! {
    products (id) {
        id -> Int4,
        uuid -> Uuid,
        title -> Varchar,
        body -> Text,
        img -> Varchar,
        kind -> Varchar,
        status -> Varchar,
        duration -> Int4,
        user_id -> Int4,
        simple -> Varchar,
    }
}

table! {
    products_tags (id) {
        id -> Int4,
        product_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    reactions (id) {
        id -> Int4,
        product_id -> Int4,
        user_id -> Int4,
        kind -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    tags (id) {
        id -> Int4,
        uuid -> Uuid,
        name -> Varchar,
        kind -> Varchar,
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
        avatar -> Nullable<Varchar>,
        bio -> Nullable<Varchar>,
    }
}

joinable!(products -> users (user_id));
joinable!(products_tags -> products (product_id));
joinable!(products_tags -> tags (tag_id));
joinable!(reactions -> products (product_id));
joinable!(reactions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    products,
    products_tags,
    reactions,
    tags,
    users,
);
