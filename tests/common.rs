use crate::rocket;
use rocket::local::Client;
use std::env;

pub fn test_client() -> Client {
    env::set_var("ROCKET_ENV", "stage");
    Client::new(rocket()).expect("valid rocket instance")
}