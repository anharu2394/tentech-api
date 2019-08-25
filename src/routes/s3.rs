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

#[derive(Serialize)]
pub struct CreatedAsset {
    url: String,
}
#[post("/upload", format = "json", data = "<new_asset>")]
pub fn upload(
    new_asset: Json<NewAsset>,
    client: State<S3Client>,
) -> Result<JsonValue, TentechError> {
    let new_asset = new_asset.into_inner().asset;
    let mut request = PutObjectRequest::default();
    request.bucket = String::from("tentech");
    request.key = new_asset.key.to_string();
    let body =
        base64::decode(&new_asset.attachment).map_err(|_| TentechError::CannotDecodeBase64)?;
    println!("{}", body.len())
    request.body = Some(ByteStream::from(body));
    client
        .put_object(request)
        .sync()
        .map(|a| json!({ "asset": CreatedAsset { url: format!("https://tentech.s3-ap-northeast-1.amazonaws.com/{}",new_asset.key)}}))
        .map_err(|_| TentechError::CannotPutS3Object)
}
