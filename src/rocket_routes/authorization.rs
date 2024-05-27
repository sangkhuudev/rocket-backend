use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};
use rocket::{http::Status, response::status::Custom, serde::json::{json, Json, Value}};
use crate::auth::{authorize_user, Credentials};
use super::{server_error, CacheConn, DbConn};
use crate::repositories::UserRepository;


#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(
    mut db: Connection<DbConn>, 
    mut cache: Connection<CacheConn>,
    credentials: Json<Credentials>
) -> Result<Value, Custom<Value>> {
    let user = match UserRepository::find_by_username(&mut db, &credentials.username).await {
        Ok(user) => user,
        Err(diesel::result::Error::NotFound) => return Err(Custom(Status::Unauthorized, json!("Wrong credentials"))),
        Err(e) => return Err(server_error(e.into())),
    };
    

    let session_id = authorize_user(&user, credentials.into_inner())
        .map_err(|_| Custom(Status::Unauthorized, json!("Wrong credentials")))?;
    // Set and store session_id in redis database
    cache.set_ex::<String, i32, ()>(
        format!("sessions/{}", session_id), 
        user.id,
        3*60*60
    ).await
    .map_err(|e| server_error(e.into()))?;

    Ok(json!({
        "token": session_id
    }))
}