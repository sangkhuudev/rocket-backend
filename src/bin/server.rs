use rocket_db_pools::Database;
use rocket_backend::rocket_routes::{
    crates::{create_crate, delete_crate, get_crate, get_crates, update_crate}, 
    rustaceans::{create_rustacean, delete_rustacean, get_rustacean, get_rustaceans, update_rustacean}, 
    DbConn
};


#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            get_rustaceans,
            get_rustacean,
            create_rustacean,
            update_rustacean,
            delete_rustacean,
            get_crates,
            get_crate,
            create_crate,
            update_crate,
            delete_crate
        ])
        .attach(DbConn::init())
        .launch()
        .await;
}
