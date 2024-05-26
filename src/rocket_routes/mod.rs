use rocket_db_pools::Database;
use rocket::{response::status::Custom, serde::json::{json, Value}};
use rocket::http::Status;

pub mod authorization;
pub mod crates;
pub mod rustaceans;

#[derive(Database)]
#[database("postgres")] // "postgres" database name in the docker-compose.yml: section app (environment)
pub struct DbConn(rocket_db_pools::diesel::PgPool);


#[derive(Database)]
#[database("redis")]  // "redis" database name in the docker-compose.yml: section app (environment)
pub struct CacheConn(rocket_db_pools::deadpool_redis::Pool);

pub fn server_error(e: Box< dyn std::error::Error>) -> Custom<Value>{
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}