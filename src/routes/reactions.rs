use crate::db;
use crate::error::TentechError;
use crate::models::user::TokenData;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewReaction {
    pub kind: String,
}

#[post(
    "/products/<id>/reaction/add",
    format = "json",
    data = "<new_reaction>"
)]
pub fn add_react(
    new_reaction: Json<NewReaction>,
    conn: db::Conn,
    token: TokenData,
    id: i32,
) -> Result<JsonValue, TentechError> {
    let new_reaction = new_reaction.into_inner();
    db::reactions::add_react(&conn, &new_reaction, &id, &token.user.id)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
        .map(|_| json!({}))
}

#[post(
    "/products/<id>/reaction/sub",
    format = "json",
    data = "<new_reaction>"
)]
pub fn sub_react(
    new_reaction: Json<NewReaction>,
    conn: db::Conn,
    token: TokenData,
    id: i32,
) -> Result<JsonValue, TentechError> {
    let new_reaction = new_reaction.into_inner();
    db::reactions::sub_react(&conn, &new_reaction, &id, &token.user.id)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
        .map(|_| json!({}))
}

#[get("/users/<id>/reactions")]
pub fn get_by_user_id(conn: db::Conn, id: i32) -> Result<JsonValue, TentechError> {
    db::reactions::get_by_user_id(&conn, &id)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
        .map(|r| json!(r))
}
