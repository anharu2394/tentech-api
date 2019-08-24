use crate::db;
use crate::error::TentechError;
use crate::models::user::TokenData;
use crate::validation::FieldValidator;
use percent_encoding::percent_decode_str;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use std::collections::HashMap;
use std::vec::Vec;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize)]
pub struct NewProduct {
    product: NewProductData,
}

#[derive(Deserialize, Validate)]
pub struct NewProductData {
    #[validate(length(min = "1", max = "33"))]
    title: Option<String>,
    #[validate(length(min = "1"))]
    body: Option<String>,
    #[validate(url)]
    img: Option<String>,
    duration: i32,
    kind: String,
    tags: Vec<i32>,
}

#[post("/products", format = "json", data = "<new_product>")]
pub fn post_products(
    new_product: Json<NewProduct>,
    conn: db::Conn,
    token: TokenData,
) -> Result<JsonValue, TentechError> {
    let new_product = new_product.into_inner().product;

    let mut extractor = FieldValidator::validate(&new_product);
    let title = extractor.extract("title", new_product.title);
    let body = extractor.extract("body", new_product.body);
    let img = extractor.extract("img", new_product.img);

    extractor
        .check()
        .map_err(|e| TentechError::ValidationFailed(e.errors))?;

    db::products::create(
        &conn,
        &title,
        &body,
        &img,
        &new_product.duration,
        &new_product.kind,
        &new_product.tags,
        &token.user.id,
    )
    .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
    .map(|pd| json!({ "product": pd }))
}

#[patch("/products/<id>", format = "json", data = "<update_product>")]
pub fn update_products(
    update_product: Json<NewProductData>,
    conn: db::Conn,
    token: TokenData,
    id: String,
) -> Result<JsonValue, TentechError> {
    let uuid = Uuid::parse_str(&id).unwrap();
    let product = db::products::find(&conn, &uuid)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))?;
    if !(product.user_id == token.user.id) {
        return Err(TentechError::Unauthorized(
            "Cannot delete other's product".to_string(),
        ));
    }
    let update_product = update_product.into_inner();

    let mut extractor = FieldValidator::validate(&update_product);
    let title = extractor.extract("title", update_product.title);
    let body = extractor.extract("body", update_product.body);
    let img = extractor.extract("img", update_product.img);

    extractor
        .check()
        .map_err(|e| TentechError::ValidationFailed(e.errors))?;
    db::products::update(
        &conn,
        &title,
        &body,
        &img,
        &update_product.duration,
        &update_product.kind,
        &update_product.tags,
        &token.user.id,
        &uuid,
    )
    .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
    .map(|pd| json!({ "product": pd }))
}

#[delete("/products/<id>")]
pub fn delete_products(
    conn: db::Conn,
    token: TokenData,
    id: String,
) -> Result<JsonValue, TentechError> {
    let uuid = Uuid::parse_str(&id).unwrap();
    let product = db::products::find(&conn, &uuid)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))?;
    if !(product.user_id == token.user.id) {
        return Err(TentechError::Unauthorized(
            "Cannot delete other's product".to_string(),
        ));
    }
    db::products::delete(&conn, &uuid)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
        .map(|_| json!({}))
}

#[get("/products/<id>")]
pub fn get(conn: db::Conn, id: String) -> Result<JsonValue, TentechError> {
    let uuid = Uuid::parse_str(&id).unwrap();
    db::products::find(&conn, &uuid)
        .and_then(|p| {
            let user = db::users::find(&conn, &p.user_id)?;
            Ok(json!({ "product": p, "user": user}))
        })
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
}

#[get("/users/<user_id>/products")]
pub fn get_by_user_id(conn: db::Conn, user_id: i32) -> Result<JsonValue, TentechError> {
    db::products::find_by_user_id(&conn, &user_id)
        .and_then(|p| Ok(json!({ "products": p })))
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
}
