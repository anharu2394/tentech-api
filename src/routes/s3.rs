use crate::error::TentechError;
use crate::models::user::TokenData;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewAsset {
    asset: NewProductData,
}

#[derive(Deserialize)]
pub struct NewAssetData {
    key: String,
    attachment: String,
}

#[post("/upload", format = "json", data = "<new_asset>")]
pub fn upload(new_asset: Json<NewAsset>) -> Result<NewAsset, TentechError> {}
