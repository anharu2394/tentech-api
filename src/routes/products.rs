use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
struct NewProduct {
    product: NewProductData,
}

#[derive(Deserialize, Validate)]
struct NewProductData {
    #[validate(length(min = "1", max = "33"))]
    title: Option<String>,
    #[validate(length(min = "1"))]
    body: Option<String>,
    #[validate(url)]
    img: Option<String>,
    duration: i32,
    kind: String,
}

#[post("/products", format = "json", data = "<new_product>")]
pub fn post_products(
    new_product: Json<NewProduct>,
    conn: db::Conn,
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
    )
    .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
    .map(|pd| json!({ "product": pd }))
}
