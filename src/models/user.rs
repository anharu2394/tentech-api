use crate::email::{send_activation_email, SendError};
use crate::schema::users;
use crate::token::{decrypt, encrypt};
use chrono::offset::Local;
use chrono::DateTime;
use chrono::Duration;
use fernet::DecryptionError;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Clone, Queryable, Serialize, Deserialize, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub activated: bool,
    pub activated_at: Option<SystemTime>,
}

#[derive(Clone, Queryable, Serialize, Deserialize)]
pub struct TokenData {
    #[serde(flatten)]
    pub user: User,
    pub expired_at: DateTime<Local>,
}

impl User {
    pub fn prepare_activate(&self) -> Result<User, SendError> {
        let token = self.generate_token();
        let encoded_token = percent_encode(token.as_bytes(), NON_ALPHANUMERIC).to_string();
        match send_activation_email(&self.email, &self.nickname, &encoded_token) {
            Some(err) => return Err(err),
            None => {}
        }
        Ok(self.clone())
    }
    pub fn generate_token(&self) -> String {
        let json = serde_json::to_string(&self.to_token_data()).unwrap();
        encrypt(&json)
    }
    pub fn to_token_data(&self) -> TokenData {
        let expired_at = Local::now() + Duration::days(1);
        TokenData {
            user: self.clone(),
            expired_at,
        }
    }
}

impl TokenData {
    pub fn decode(token: String) -> Result<TokenData, DecryptionError> {
        let bytes_text = decrypt(&token)?;
        let string_text = String::from_utf8(bytes_text).unwrap();
        Ok(serde_json::from_str::<TokenData>(&string_text).unwrap())
    }
    pub fn check_expired(&self) -> bool {
        Local::now() < self.expired_at
    }
}

fn check_valid(key: &str) -> Result<TokenData, ()> {
    TokenData::decode(key.to_string())
        .map_err(|_| ())
        .and_then(|token_data| {
            if token_data.check_expired() {
                Ok(token_data)
            } else {
                Err(())
            }
        })
}

#[derive(Debug)]
pub enum TokenError {
    BadCount,
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for TokenData {
    type Error = TokenError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, Self::Error::Missing)),
            1 => match check_valid(keys[0]) {
                Ok(token_data) => Outcome::Success(token_data),
                Err(_) => Outcome::Failure((Status::BadRequest, Self::Error::Invalid)),
            },
            _ => Outcome::Failure((Status::BadRequest, Self::Error::BadCount)),
        }
    }
}
