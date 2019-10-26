use tentech_api::rocket;
use tentech_api::db;
use tentech_api::test_establish_connection;
use tentech_api::schema::users;
use ::rocket::local::Client;
use ::rocket::Response;
use std::env;
use serde_json::Value;
use diesel::dsl::*;
use diesel::prelude::*;

pub const PASSWORD: &'static str = "passpassword";

#[macro_export]
macro_rules! json_string {
    ($value:tt) => {
        serde_json::to_string(&serde_json::json!($value)).expect("cannot json stringify")
    };
}

pub fn test_client() -> Client {
    env::set_var("ROCKET_ENV", "stage");
    Client::new(rocket()).expect("valid rocket instance")
}

pub fn delete_all_users() {
    let conn = test_establish_connection();
    db::users::delete_all(&conn).expect("failed");
    let count = users::table.select(count_star()).first(&conn);
    assert_eq!(Ok(0), count);
}

pub fn see_user(name: &String) {
    match db::users::find_by_username(&test_establish_connection(), name) {
        Ok(r) => println!("{}", json_string!(r)),
        Err(e) => println!("{:?}", e)
    }
}

pub fn create_user(name: &str) {
    db::users::create(&test_establish_connection(), &(name.to_owned() + "_test"), &(name.to_owned() + "_test"),&(name.to_owned() + "_test@test.com"),PASSWORD).expect("failed");
}

pub fn user_json(name: &str) -> Value{
    serde_json::json!({"username": &(name.to_owned() + "_test"), "nickname": &(name.to_owned() + "_test"), "email":&(name.to_owned() + "_test@test.com"), "password": PASSWORD})
}
pub fn print_response(response: &mut Response) {
    println!("{}", response.body_string().unwrap())
}