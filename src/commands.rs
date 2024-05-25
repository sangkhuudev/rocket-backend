use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::{models::NewUser, repositories::UserRepository};


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
    role_code: Vec<String>
) {
    let mut conn = load_database_connection().await;
    let new_user = NewUser { username, password};
    let user = UserRepository::create(&mut conn, new_user).await;
    
    match user {
        Ok(user) => println!("User created successfully: {:?}", user),
        Err(err) => eprintln!("Error creating user: {:?}", err),
    }
}