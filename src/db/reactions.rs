use crate::db;
use crate::error::TentechError;
use crate::models::product::Product;
use crate::models::reaction::Reaction;
use crate::models::user::User;
use crate::routes::reactions::NewReaction;
use crate::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use std::time::SystemTime;
use std::vec::Vec;

pub fn add_react(
    conn: &PgConnection,
    reaction: &NewReaction,
    product_id: &i32,
    user_id: &i32,
) -> Result<(), TentechError> {
    let current_count = reactions::table
        .filter(reactions::product_id.eq(product_id))
        .filter(reactions::user_id.eq(user_id))
        .filter(reactions::kind.eq(reaction.kind.to_string()))
        .select(reactions::id)
        .load::<i32>(conn)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))?
        .len() as i32;
    if current_count > 4 {
        return Err(TentechError::CannotReactTooMany);
    }
    diesel::insert_into(reactions::table)
        .values((
            reactions::kind.eq(reaction.kind.to_string()),
            reactions::product_id.eq(product_id),
            reactions::user_id.eq(user_id),
            reactions::created_at.eq(SystemTime::now()),
        ))
        .execute(conn)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))?;
    Ok(())
}

pub fn sub_react(
    conn: &PgConnection,
    reaction: &NewReaction,
    product_id: &i32,
    user_id: &i32,
) -> Result<(), TentechError> {
    let current_count = reactions::table
        .filter(reactions::product_id.eq(product_id))
        .filter(reactions::user_id.eq(user_id))
        .filter(reactions::kind.eq(reaction.kind.to_string()))
        .select(reactions::id)
        .load::<i32>(conn)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))?
        .len() as i32;
    if current_count < 1 {
        return Err(TentechError::CannotReactTooMany);
    }
    let delete_id = reactions::table
        .select(reactions::id)
        .filter(reactions::product_id.eq(product_id))
        .filter(reactions::user_id.eq(user_id))
        .filter(reactions::kind.eq(reaction.kind.to_string()))
        .order(reactions::id.desc())
        .first::<i32>(conn)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))?;
    diesel::delete(reactions::table.find(delete_id))
        .execute(conn)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))?;
    Ok(())
}

pub fn get_by_product_id(conn: &PgConnection, product_id: &i32) -> Result<Vec<Reaction>, Error> {
    reactions::table
        .filter(reactions::product_id.eq(product_id))
        .load::<Reaction>(conn)
}

pub fn get_by_user_id(
    conn: &PgConnection,
    user_id: &i32,
) -> Result<Vec<(Product, Reaction, User)>, Error> {
    let resources: Vec<_> = users::table
        .inner_join(products::table.inner_join(reactions::table))
        .filter(users::id.eq(user_id))
        .order(reactions::created_at.desc())
        .limit(20)
        .load::<(User, (Product, Reaction))>(conn)?;
    let new_resources: Vec<(Product, Reaction, User)> = resources
        .iter()
        .map(|r| {
            let by = db::users::find(conn, &(&r.1).1.user_id).unwrap();
            ((r.1).0.clone(), (r.1).1.clone(), by)
        })
        .collect();
    Ok(new_resources)
}
