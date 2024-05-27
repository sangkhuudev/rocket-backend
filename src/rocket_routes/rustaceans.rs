use crate::{models::{NewRustacean, Rustacean, User}, repositories::RustaceanRepository};
use rocket_db_pools::Connection;
use rocket::{response::status::{Custom, NoContent}, serde::json::{json, Json, Value}};
use rocket::http::Status;
use diesel::result::Error as DieselError;

use crate::rocket_routes::DbConn;

use super::server_error;


#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find_multiple(&mut db, 100).await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/rustaceans/<id>")]
pub async fn get_rustacean(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<Value, Custom<Value>> {
    match RustaceanRepository::find(&mut db, id).await {
        Ok(rustacean) => Ok(json!(rustacean)),
        Err(e) => match e {
            DieselError::NotFound => Err(Custom(Status::NotFound, json!({"error": "Not Found"}))),
            _ => Err(Custom(Status::InternalServerError, json!({"error": "Internal Server Error"}))),
        },
    }
}

#[rocket::post("/rustaceans", format="json", data="<new_rustacean>")]
pub async fn create_rustacean(
    mut db: Connection<DbConn>, 
    new_rustacean: Json<NewRustacean>,
    _user: User
) -> Result<Custom<Value>, Custom<Value>> {
    RustaceanRepository::create(&mut db, new_rustacean.into_inner()).await
        .map(|rustacean| Custom(Status::Created, json!(rustacean)))
        .map_err(|e| server_error(e.into()))
}


#[rocket::put("/rustaceans/<id>", format="json", data="<rustacean>")]
pub async fn update_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
    _user: User,
    rustacean: Json<Rustacean>,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::update(&mut db, id, rustacean.into_inner()).await
        .map(|rustacean| json!(rustacean))
        .map_err(|e| server_error(e.into()))
}


#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<NoContent, Custom<Value>> {
    RustaceanRepository::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}