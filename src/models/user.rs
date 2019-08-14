use serde::Serialize;
use std::time::SystemTime;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub token: String,
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub activated: bool,
    pub activated_at: Option<SystemTime>,
    pub expired_at: SystemTime,
}

impl User {
    pub fn prepare_activate(&self) {}
}
