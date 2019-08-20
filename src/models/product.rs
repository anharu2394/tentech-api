use crate::schema::products;
use serde::{Deserialize, Serialize};

#[derive(Clone, Queryable, Serialize, Deserialize, Identifiable)]
#[table_name = "products"]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub img: String,
    pub kind: String,
    pub duration: i32,
    pub user_id: i32,
}
