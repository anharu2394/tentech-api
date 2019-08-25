use crate::error::TentechError;
use crate::models::user::TokenData;
use base64;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rusoto_core::ByteStream;
use rusoto_s3::{GetObjectRequest, PutObjectOutput, PutObjectRequest, S3Client, S3};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NewAsset {
    asset: NewAssetData,
}

#[derive(Deserialize)]
pub struct NewAssetData {
    key: String,
    attachment: String,
}

#[post("/upload", format = "json", data = "<new_asset>")]
pub fn upload(new_asset: Json<NewAsset>) -> Result<NewAsset, TentechError> {}
