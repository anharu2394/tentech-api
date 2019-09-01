use crate::db;
use crate::error::TentechError;
use crate::models::product::Product;
use crate::models::tag::ProductTag;
use crate::schema::{products, products_tags, tags};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Insertable, AsChangeset)]
#[primary_key(uuid)]
#[table_name = "products"]
pub struct NewProduct<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub simple: &'a str,
    pub img: &'a str,
    pub duration: &'a i32,
    pub kind: &'a str,
    pub status: &'a str,
    pub user_id: &'a i32,
    pub uuid: &'a Uuid,
}

pub fn create(
    conn: &PgConnection,
    title: &str,
    body: &str,
    simple: &str,
    img: &str,
    duration: &i32,
    kind: &str,
    status: &str,
    tags: &Vec<i32>,
    user_id: &i32,
) -> Result<Product, Error> {
    let new_product = &NewProduct {
        title,
        body,
        simple,
        img,
        duration,
        kind,
        status,
        user_id,
        uuid: &Uuid::new_v4(),
    };

    let product = diesel::insert_into(products::table)
        .values(new_product)
        .get_result::<Product>(conn)?;
    db::tags::entry_to_product(conn, product.id, tags.to_vec());
    Ok(product)
}

pub fn update(
    conn: &PgConnection,
    title: &str,
    body: &str,
    simple: &str,
    img: &str,
    duration: &i32,
    kind: &str,
    status: &str,
    tags: &Vec<i32>,
    user_id: &i32,
    uuid: &Uuid,
) -> Result<Product, Error> {
    let new_product = &NewProduct {
        title,
        body,
        simple,
        img,
        duration,
        kind,
        status,
        user_id,
        uuid,
    };

    let product = diesel::update(products::table.filter(products::uuid.eq(uuid)))
        .set(new_product)
        .get_result::<Product>(conn)?;
    db::tags::delete_by_product_id(conn, product.id)?;
    db::tags::entry_to_product(conn, product.id, tags.to_vec())?;
    Ok(product)
}

pub fn find(conn: &PgConnection, id: &Uuid) -> Result<Product, Error> {
    products::table
        .filter(products::uuid.eq(id))
        .first::<Product>(conn)
}

pub fn find_by_id(conn: &PgConnection, id: &i32) -> Result<Product, Error> {
    products::table.find(id).first::<Product>(conn)
}

pub fn find_by_user_id(conn: &PgConnection, id: &i32) -> Result<Vec<Product>, Error> {
    products::table
        .filter(products::user_id.eq(id))
        .load::<Product>(conn)
}

pub fn find_by_tag_name(conn: &PgConnection, name: &String) -> Result<Vec<Product>, Error> {
    let tag_id = tags::table
        .select(tags::id)
        .filter(tags::name.eq(name))
        .first::<i32>(conn)?;
    products_tags::table
        .inner_join(products::table)
        .filter(products_tags::tag_id.eq(tag_id))
        .load::<(ProductTag, Product)>(conn)
        .map(|p| {
            let new_p: Vec<_> = p.iter().map(|i| i.1.clone()).collect();
            new_p
        })
}

pub fn delete(conn: &PgConnection, id: &Uuid) -> Result<usize, Error> {
    diesel::delete(products::table.filter(products::uuid.eq(id))).execute(conn)
}
pub fn recent(conn: &PgConnection) -> Result<Vec<Product>, Error> {
    products::table
        .order(products::id.desc())
        .load::<Product>(conn)
}
pub fn popular(conn: &PgConnection) -> Result<Vec<Product>, Error> {
    diesel::sql_query(
        "SELECT products.* FROM reactions INNER JOIN products on products.id = reactions.product_id GROUP BY product_id, products.id ORDER BY count(product_id) DESC",
    )
        .load(conn)
}
