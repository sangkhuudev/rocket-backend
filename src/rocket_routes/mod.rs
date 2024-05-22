use rocket_db_pools::Database;

pub mod rustaceans;
pub mod crates;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);
