use crate::db;
use crate::error::TentechError;
use crate::models::user::TokenData;
use rocket_contrib::json::{Json, JsonValue};

#[get("/tags")]
pub fn get_all(conn: db::Conn) -> Result<JsonValue, TentechError> {
    db::tags::get_all(&conn)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
        .map(|t| json!({ "tags": t }))
}
