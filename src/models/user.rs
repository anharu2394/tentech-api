use crate::email::{send_activation_email, SendError};
use crate::token::{decrypt, encrypt};
use chrono::offset::Local;
use chrono::DateTime;
use chrono::Duration;
use fernet::DecryptionError;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Clone, Queryable, Serialize, Deserialize)]
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
        match send_activation_email(&self.email, &self.nickname, &token) {
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
}
