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
    #[validate(length(min = "1"))]
    simple: Option<String>,
    #[validate(url)]
    img: Option<String>,
    duration: i32,
    kind: String,
    status: String,
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
    let simple = extractor.extract("simple", new_product.simple);
    let img = extractor.extract("img", new_product.img);

    extractor
        .check()
        .map_err(|e| TentechError::ValidationFailed(e.errors))?;

    db::products::create(
        &conn,
        &title,
        &body,
        &simple,
        &img,
        &new_product.duration,
        &new_product.kind,
        &new_product.status,
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
    let simple = extractor.extract("simple", update_product.simple);
    let img = extractor.extract("img", update_product.img);

    extractor
        .check()
        .map_err(|e| TentechError::ValidationFailed(e.errors))?;
    db::products::update(
        &conn,
        &title,
        &body,
        &simple,
        &img,
        &update_product.duration,
        &update_product.kind,
        &update_product.status,
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
            let tag_ids = db::tags::get_by_product_id(&conn, &p.id)?;
            let reactions = db::reactions::get_by_product_id(&conn, &p.id)?;
            Ok(json!({ "product": p, "user": user, "tag_ids": tag_ids, "reactions": reactions }))
        })
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
}

#[get("/users/<user_id>/products")]
pub fn get_by_user_id(conn: db::Conn, user_id: i32) -> Result<JsonValue, TentechError> {
    db::products::find_by_user_id(&conn, &user_id)
        .and_then(|ps| {
            let products_with_tags_and_reactions: Vec<_> = ps
                .iter()
                .map(|p| {
                    let tag_ids = db::tags::get_by_product_id(&conn, &p.id).unwrap();
                    let reactions = db::reactions::get_by_product_id(&conn, &p.id).unwrap();
                    let mut json_tag = json!(p).as_object_mut().unwrap().clone();
                    json_tag.insert("tag_ids".to_string(), json!(tag_ids).into());
                    json_tag.insert("reactions".to_string(), json!(reactions).into());
                    json_tag
                })
                .collect();
            Ok(json!({ "products": products_with_tags_and_reactions }))
        })
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
}

#[get("/products/recent")]
pub fn recent(conn: db::Conn) -> Result<JsonValue, TentechError> {
    db::products::recent(&conn)
        .and_then(|ps| {
            let products_with_tags_and_reactions: Vec<_> = ps
                .iter()
                .map(|p| {
                    let user = db::users::find(&conn, &p.user_id).unwrap();
                    let tag_ids = db::tags::get_by_product_id(&conn, &p.id).unwrap();
                    let reactions = db::reactions::get_by_product_id(&conn, &p.id).unwrap();
                    let mut json_tag = json!(p).as_object_mut().unwrap().clone();
                    json_tag.insert("tag_ids".to_string(), json!(tag_ids).into());
                    json_tag.insert("reactions".to_string(), json!(reactions).into());
                    json_tag.insert("user".to_string(), json!(user).into());
                    json_tag
                })
                .collect();
            Ok(json!({ "products": products_with_tags_and_reactions }))
        })
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
}

#[get("/products/popular")]
pub fn popular(conn: db::Conn) -> Result<JsonValue, TentechError> {
    db::products::popular(&conn)
        .and_then(|ps| {
            let products_with_tags_and_reactions: Vec<_> = ps
                .iter()
                .map(|p| {
                    let user = db::users::find(&conn, &p.user_id).unwrap();
                    let tag_ids = db::tags::get_by_product_id(&conn, &p.id).unwrap();
                    let reactions = db::reactions::get_by_product_id(&conn, &p.id).unwrap();
                    let mut json_tag = json!(p).as_object_mut().unwrap().clone();
                    json_tag.insert("tag_ids".to_string(), json!(tag_ids).into());
                    json_tag.insert("reactions".to_string(), json!(reactions).into());
                    json_tag.insert("user".to_string(), json!(user).into());
                    json_tag
                })
                .collect();
            Ok(json!({ "products": products_with_tags_and_reactions }))
        })
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
}
