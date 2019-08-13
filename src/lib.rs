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

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![routes::users::post_users,])
        .attach(db::Conn::fairing())
}
