use crate::db;
use crate::models::product::Product;
use crate::models::user::User;
use crate::schema::reactions;
use diesel::associations;
use diesel::pg::PgConnection;
use diesel::Identifiable;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::time::SystemTime;

#[derive(Identifiable, Clone, Queryable, Serialize, Deserialize, Associations)]
#[belongs_to(parent = "User")]
#[primary_key(id)]
#[table_name = "reactions"]
pub struct Reaction {
    pub id: i32,
    pub product_id: i32,
    pub user_id: i32,
    pub kind: String,
    pub created_at: SystemTime,
}
