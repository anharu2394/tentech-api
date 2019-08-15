use crate::models::user::User;
use crate::schema::users;
use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use serde::Deserialize;
use std::time::SystemTime;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub nickname: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub activated: &'a bool,
}

pub enum UserCreationError {
    DuplicatedEmail,
    DuplicatedUsername,
}

impl From<Error> for UserCreationError {
    fn from(err: Error) -> UserCreationError {
        if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            match info.constraint_name() {
                Some("users_username_key") => return UserCreationError::DuplicatedUsername,
                Some("users_email_key") => return UserCreationError::DuplicatedEmail,
                _ => {}
            }
        }
        panic!("Error creating user: {:?}", err)
    }
}

pub fn create(
    conn: &PgConnection,
    username: &str,
    nickname: &str,
    email: &str,
    password: &str,
) -> Result<User, UserCreationError> {
    let hash = &scrypt_simple(password, &ScryptParams::new(14, 8, 1)).expect("hash error");

    let new_user = &NewUser {
        username,
        nickname,
        email,
        password: hash,
        activated: &false,
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)
        .map_err(Into::into)
}

pub fn delete_all(conn: &PgConnection) -> Result<usize, Error> {
    diesel::delete(users::table).execute(conn)
}

pub fn activate(conn: &PgConnection, target: &User) -> Result<usize, Error> {
    diesel::update(target)
        .set((
            users::activated.eq(true),
            users::activated_at.eq(SystemTime::now()),
        ))
        .execute(conn)
}

pub fn find(conn: &PgConnection, id: &i32) -> Result<User, Error> {
    users::table.find(id).first::<User>(conn)
}
