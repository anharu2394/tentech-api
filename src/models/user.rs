use crate::token::encrypt;
use serde::Serialize;
use std::time::SystemTime;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub activated: bool,
    pub activated_at: Option<SystemTime>,
}

impl User {
    pub fn prepare_activate(&self) -> Result<User, Error> {
        let token = self.generate_token();
    }
    pub fn generate_token(&self) -> String {
        let json = serde_json::to_string(&self).unwrap();
        encrypt(&json)
    }
}
