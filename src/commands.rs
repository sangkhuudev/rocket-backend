use chrono::{Datelike, Utc};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use dotenv::dotenv;
use std::env;
use std::str::FromStr;
use tera::{Context, Tera};

use crate::{
    auth::hash_password,
    mail::HtmlMailer,
    models::{NewUser, RoleCode},
    repositories::{CrateRepository, RoleRepositoty, UserRepository},
};

fn load_template_engine() -> Tera {
    Tera::new("templates/**/*.html").expect("Can not load template engine")
}
async fn load_database_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Cannot connect to Postgres")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut conn = load_database_connection().await;
    let hashed_password = hash_password(password).expect("Cannot hash the password");
    let new_user = NewUser {
        username,
        password: hashed_password,
    };
    // Convert Vec<String> into RoleCode enum
    let roles_enum = role_codes
        .iter()
        .map(|v| RoleCode::from_str(v.as_str()).unwrap())
        .collect();
    let user = UserRepository::create(&mut conn, new_user, roles_enum).await;

    match user {
        Ok(user) => {
            println!("User created successfully: {:?}", user);
            let roles = RoleRepositoty::find_by_user(&mut conn, &user)
                .await
                .unwrap();
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

pub async fn delete_user(id: i32) {
    let mut conn = load_database_connection().await;
    let user_id = UserRepository::delete(&mut conn, id).await;

    match user_id {
        Ok(_) => {
            println!("User deleted successfully");
        }
        Err(err) => eprintln!("Error creating user: {:?}", err),
    }
}

pub async fn digest_send(email: String, hours_since: i32) {
    // Load environment variables from .env file
    dotenv().ok();

    let mut conn = load_database_connection().await;
    let crates = CrateRepository::find_since(&mut conn, hours_since)
        .await
        .unwrap();
    let tera = load_template_engine();
    let year = Utc::now().year();
    if !crates.is_empty() {
        println!("Sending emails in {} crates", crates.len());
        let mut context = Context::new();
        context.insert("crates", &crates);
        context.insert("year", &year);

        let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let smtp_port = env::var("SMTP_PORT").expect("SMTP_PORT must be set");
        let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

        let mailer = HtmlMailer {
            template_engine: tera,
            smtp_host,
            smtp_port,
            smtp_username,
            smtp_password,
        };

        match mailer.send(email, "email/digest.html", context) {
            Ok(_) => println!("Digest sent successfully"),
            Err(err) => eprintln!("Error sending digest: {:?}", err),
        }
    } else {
        println!("No crates found to send.");
    }
}
