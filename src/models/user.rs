use crate::email::{send_activation_email, SendError};
use crate::token::encrypt;
use chrono::offset::Utc;
use chrono::DateTime;
use serde::Serialize;
use std::time::Duration;
use std::time::SystemTime;

#[derive(Clone, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub activated: bool,
    pub activated_at: Option<SystemTime>,
}

#[derive(Clone, Queryable, Serialize)]
pub struct TokenData {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub activated: bool,
    pub activated_at: Option<SystemTime>,
    pub expired_at: SystemTime,
}

impl User {
    pub fn prepare_activate(&self) -> Result<User, SendError> {
        let token = self.generate_token();
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
        let expired_at = SystemTime::now() + Duration::from_secs(86400);
        TokenData {
            expired_at,
            id: self.id,
            username: self.username.to_string(),
            nickname: self.nickname.to_string(),
            email: self.email.to_string(),
            password: self.password.to_string(),
            activated: self.activated,
            activated_at: None,
        }
    }
}
