
use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::{auth::hash_password, models::NewUser, repositories::{RoleRepositoty, UserRepository}};


async fn load_database_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Cannot connect to Postgres")
}

pub async fn create_user(
    username: String,
    password: String,
    role_codes: Vec<String>
) {
    let mut conn = load_database_connection().await;
    let hashed_password = hash_password(password).expect("Cannot hash the password");
    let new_user = NewUser { username, password: hashed_password};
    let user = UserRepository::create(&mut conn, new_user, role_codes).await;

    match user {
        Ok(user) => {
            println!("User created successfully: {:?}", user);
            let roles = RoleRepositoty::find_by_user(&mut conn, &user).await.unwrap();
            println!("Roles assigned: {:?}", roles)
        }
        Err(err) => eprintln!("Error creating user: {:?}", err),
    }
}

pub async fn list_users() {
    let mut conn = load_database_connection().await;
    let users = UserRepository::find_with_roles(&mut conn).await.unwrap();

    for user in users {
        println!("User: {:?}", user);
    }
}

pub async fn delete_user(id : i32) {
    let mut conn = load_database_connection().await;
    let user_id = UserRepository::delete(&mut conn, id).await;

    match user_id {
        Ok(_) => {
            println!("User deleted successfully");
        }
        Err(err) => eprintln!("Error creating user: {:?}", err),
    } 
}