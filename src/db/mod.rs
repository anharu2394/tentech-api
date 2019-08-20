use rocket_contrib::databases::diesel;

pub mod products;
pub mod users;

#[database("diesel_postgres_pool")]
pub struct Conn(diesel::PgConnection);
