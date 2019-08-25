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

pub mod db;
mod email;
mod error;
mod models;
mod routes;
mod s3;
mod schema;
mod token;
mod validation;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method, Status};
use rocket::{get, routes};
use rocket::{Request, Response};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error, Guard, Responder};
use std::env;
use std::io::Cursor;

pub fn test_establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_TEST_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
pub fn rocket() -> rocket::Rocket {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();
    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::users::post_users,
                routes::users::activate,
                routes::users::login,
                routes::users::get,
                routes::users::validate,
                routes::products::post_products,
                routes::products::update_products,
                routes::products::delete_products,
                routes::products::get,
                routes::products::get_by_user_id,
            ],
        )
        .attach(cors)
        .attach(db::Conn::fairing())
        .manage(s3::initial_s3_client())
}
