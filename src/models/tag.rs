use crate::models::product::Product;
use crate::schema::products_tags;
use crate::schema::tags;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Queryable, Serialize, Deserialize, Identifiable)]
#[table_name = "tags"]
pub struct Tag {
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub kind: String,
}

#[derive(Clone, Queryable, Serialize, Deserialize, Identifiable, Associations)]
#[belongs_to(parent = "Tag")]
#[belongs_to(parent = "Product")]
#[table_name = "products_tags"]
pub struct ProductTag {
    pub id: i32,
    pub tag_id: i32,
    pub product_id: i32,
}
