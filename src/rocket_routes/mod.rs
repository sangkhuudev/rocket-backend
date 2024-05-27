use rocket::{request::{self, Outcome, FromRequest}, response::status::Custom, serde::json::{json, Value}};
use rocket::http::Status;
use rocket::Request;
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection, Database};
use crate::{models::{RoleCode, User}, repositories::{RoleRepositoty, UserRepository}};
use rocket::outcome::try_outcome;

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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Authorization: Bearer session_id_128_characters_long
        let session_header = req.headers().get_one("Authorization")
            .and_then(|v| {
                let parts: Vec<&str> = v.split_whitespace().collect();
                if parts.len() == 2 && parts[0] == "Bearer" {
                    Some(parts[1])
                } else {
                    None
                }
            });

        let session_id = match session_header {
            Some(session_id) => session_id,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        let mut cache = try_outcome!(req.guard::<Connection<CacheConn>>().await.map_error(|(status, _)| (status, ())));
        let mut db = try_outcome!(req.guard::<Connection<DbConn>>().await.map_error(|(status, _)| (status, ())));

        let user_id: i32 = match cache.get::<String, i32>(format!("sessions/{}", session_id)).await {
            Ok(user_id) => user_id,
            Err(_) => return Outcome::Error((Status::Unauthorized, ())),
        };

        match UserRepository::find(&mut db, user_id).await {
            Ok(user) => Outcome::Success(user),
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

pub struct EditorUser(User);
#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Retrieve a connection to the database
        let mut db = try_outcome!(req.guard::<Connection<DbConn>>().await.map_error(|(status, _)| (status, ())));
        
        // Retrieve the User object from the request
        let user = try_outcome!(req.guard::<User>().await.map_error(|(status, _)| (status, ())));
        
        // Find roles for the user
        match RoleRepositoty::find_by_user(&mut db, &user).await {
            Ok(roles) => {
                rocket::info!("Roles assigned: {:?}", roles);
                
                // Check if the user has either Editor or Admin roles
                let is_editor = roles.iter().any(|r| {
                    match r.code {
                        RoleCode::Editor | RoleCode::Admin => true,
                        _ => false,
                    }
                });

                rocket::info!("Is editor: {:?}", is_editor);

                if is_editor {
                    return Outcome::Success(EditorUser(user));
                }
            },
            Err(e) => {
                rocket::error!("Error fetching roles: {:?}", e);
            }
        }
        
        // If the user is not an editor, return Unauthorized
        Outcome::Error((Status::Unauthorized, ()))
    }
}