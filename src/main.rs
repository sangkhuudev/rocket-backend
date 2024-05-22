use rocket_db_pools::Database;

mod models;
mod schema;
mod repositories;

#[derive(Database)]
#[database("postgres")]
struct DbConn(rocket_db_pools::diesel::PgPool);

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![

        ])
        .attach(DbConn::init())
        .launch()
        .await;
}
