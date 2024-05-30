use std::str::FromStr;
use chrono::{Datelike, Utc};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use lettre::{message::{header::ContentType, MessageBuilder}, transport::smtp::authentication::Credentials, SmtpTransport};
use lettre::Transport;
use tera::{Context, Tera};

use crate::{auth::hash_password, models::{NewUser, RoleCode}, repositories::{CrateRepository, RoleRepositoty, UserRepository}};

fn load_template_engine() -> Tera {
    Tera::new("templates/**/*.html")
        .expect("Can not load template engine")
}
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
    // Convert Vec<String> into RoleCode enum
    let roles_enum = role_codes.iter().map(|v| RoleCode::from_str(v.as_str()).unwrap()).collect();
    let user = UserRepository::create(&mut conn, new_user, roles_enum).await;

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

pub async fn digest_send(email: String, id : i32) {
    let mut conn = load_database_connection().await;
    let crates = CrateRepository::find_since(&mut conn, id).await.unwrap();
    let tera = load_template_engine();
    let year = Utc::now().year();
    if crates.len() > 0 {
        println!("Sending emails in {} crates", crates.len());
        // This is used as variables in digest.html 
        let mut context = Context::new();
        context.insert("crates", &crates);
        context.insert("year", &year);

        let html_body = tera.render("email/digest.html", &context).unwrap();
        // Use bulder pattern here to create message
        let message = MessageBuilder::new()
            .subject("Rocket backend digest")
            .from("from@example.com".parse().unwrap())
            .to(email.parse().unwrap())
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .unwrap();
        // Load SMTP variables from Docker compose file
        let smtp_host = std::env::var("SMTP_HOST")
            .expect("SMTP_HOST must be set in Docker compose");
        let smtp_port = std::env::var("SMTP_PORT")
            .expect("SMTP_PORT must be set in Docker compose");
        let smtp_username = std::env::var("SMTP_USERNAME")
            .expect("SMTP_USERNAME must be set in Docker compose");
        let smtp_password = std::env::var("SMTP_PASSWORD")
            .expect("SMTP_PASSWORD must be set in Docker compose");
        let credentials = Credentials::new(smtp_username, smtp_password);
        let mailer = SmtpTransport::builder_dangerous(&smtp_host)
            .port(smtp_port.parse().unwrap())
            .credentials(credentials)
            .build();

        match mailer.send(&message) {
            Ok(_) => println!("Email sent successfully"),
            Err(e) => eprintln!("Error sending email: {:?}", e),
        }
        
    }
}