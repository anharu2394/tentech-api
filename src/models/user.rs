use crate::email::{send_activation_email, SendError};
use crate::token::encrypt;
use chrono::offset::Local;
use chrono::DateTime;
use chrono::Duration;
use serde::Serialize;
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
    #[serde(flatten)]
    pub user: User,
    pub expired_at: DateTime<Local>,
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
        let expired_at = Local::now() + Duration::days(1);
        TokenData {
            user: self.clone(),
            expired_at,
        }
    }
}
