use crate::models::user::User;
use crate::schema::products;
use diesel::associations;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug, Clone, Queryable, Serialize, Deserialize, Identifiable, Associations, QueryableByName,
)]
#[belongs_to(parent = "User")]
#[table_name = "products"]
pub struct Product {
    pub id: i32,
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
    pub img: String,
    pub kind: String,
    pub status: String,
    pub duration: i32,
    pub user_id: i32,
    pub simple: String,
}
