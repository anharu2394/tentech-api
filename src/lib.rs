#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use validator;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel;

mod db;
mod errors;
mod models;
mod routes;
mod schema;
mod token;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn test_establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_TEST_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![routes::users::post_users,])
        .attach(db::Conn::fairing())
}
