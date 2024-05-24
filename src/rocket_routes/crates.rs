use rocket_db_pools::Connection;
use rocket::{response::status::{Custom, NoContent}, serde::json::{json, Json, Value}};
use rocket::http::Status;
use diesel::result::Error as DieselError;
use crate::{models::{Crate, NewCrate}, repositories::CrateRepository, DbConn};

use super::server_error;


#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    CrateRepository::find_multiple(&mut db, 100).await
        .map(|crates| json!(crates))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/crates/<id>")]
pub async fn get_crate(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    match CrateRepository::find(&mut db, id).await {
        Ok(a_crate) => Ok(json!(a_crate)),
        Err(e) => match e {
            DieselError::NotFound => Err(Custom(Status::NotFound, json!({"error": "Not Found"}))),
            _ => Err(Custom(Status::InternalServerError, json!({"error": "Internal Server Error"}))),
        },
    }
}

#[rocket::post("/crates", format="json", data="<new_crate>")]
pub async fn create_crate(
    mut db: Connection<DbConn>, 
    new_crate: Json<NewCrate>,
) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::create(&mut db, new_crate.into_inner()).await
        .map(|a_crate| Custom(Status::Created, json!(a_crate)))
        .map_err(|e| server_error(e.into()))
}


#[rocket::put("/crates/<id>", format="json", data="<a_crate>")]
pub async fn update_crate(
    mut db: Connection<DbConn>,
    id: i32,
    a_crate: Json<Crate>,
) -> Result<Value, Custom<Value>> {
    CrateRepository::update(&mut db, id, a_crate.into_inner()).await
        .map(|a_crate| json!(a_crate))
        .map_err(|e| server_error(e.into()))
}


#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>> {
    CrateRepository::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}