use crate::error::TentechError;
use crate::models::product::Product;
use crate::schema::products;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use std::time::SystemTime;

#[derive(Insertable)]
#[table_name = "products"]
pub struct NewProduct<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub img: &'a str,
    pub duration: &'a i32,
    pub kind: &'a str,
    pub user_id: &'a i32,
}

pub fn create(
    conn: &PgConnection,
    title: &str,
    body: &str,
    img: &str,
    duration: &i32,
    kind: &str,
    user_id: &i32,
) -> Result<Product, Error> {
    let new_product = &NewProduct {
        title,
        body,
        img,
        duration,
        kind,
        user_id,
    };

    diesel::insert_into(products::table)
        .values(new_product)
        .get_result::<Product>(conn)
}
