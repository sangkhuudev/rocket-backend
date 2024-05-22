use rocket_db_pools::Connection;
use rocket::{response::status::{Custom, NoContent}, serde::json::{json, Json, Value}};
use rocket::http::Status;

use crate::{models::{Crate, NewCrate}, repositories::CrateRepository, DbConn};


#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    CrateRepository::find_multiple(&mut db, 100).await
        .map(|crates| json!(crates))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error to parse Crates")))
}

#[rocket::get("/crates/<id>")]
pub async fn get_crate(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    CrateRepository::find(&mut db, id).await
        .map(|a_crate| json!(a_crate))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error to parse crates")))
}

#[rocket::post("/crates", format="json", data="<new_crate>")]
pub async fn create_crate(
    mut db: Connection<DbConn>, 
    new_crate: Json<NewCrate>,
) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::create(&mut db, new_crate.into_inner()).await
        .map(|a_crate| Custom(Status::Created, json!(a_crate)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error to parse crate")))
}


#[rocket::put("/crates/<id>", format="json", data="<a_crate>")]
pub async fn update_crate(
    mut db: Connection<DbConn>,
    id: i32,
    a_crate: Json<Crate>,
) -> Result<Value, Custom<Value>> {
    CrateRepository::update(&mut db, id, a_crate.into_inner()).await
        .map(|a_crate| json!(a_crate))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error to parse Rustaceans")))
}


#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>> {
    CrateRepository::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error to parse Rustaceans")))
}