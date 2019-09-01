use crate::error::TentechError;
use crate::models::user::User;
use crate::routes::users::UpdateUserData;
use crate::schema::users;
use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
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
) -> Result<User, Error> {
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
}

pub fn update(conn: &PgConnection, id: &i32, target: &UpdateUserData) -> Result<User, Error> {
    diesel::update(users::table.find(id))
        .set(target)
        .get_result::<User>(conn)
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

pub fn find_by_username(conn: &PgConnection, name: &String) -> Result<User, Error> {
    users::table
        .filter(users::username.eq(name))
        .first::<User>(conn)
}

pub fn login(conn: &PgConnection, email: &String, password: &String) -> Result<User, TentechError> {
    let target = users::table
        .filter(users::email.eq(email))
        .first::<User>(conn)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))?;
    scrypt_check(&password, &target.password)
        .map_err(|_| TentechError::CannotVerifyPassword)
        .and_then(|is_same| {
            if !is_same {
                Err(TentechError::CannotVerifyPassword)
            } else {
                Ok(target)
            }
        })
}
