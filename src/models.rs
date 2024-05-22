use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::{rustaceans, crates};

#[derive(Debug, Clone, Queryable, )]
pub struct Rustacean {
    pub id : i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime
}


#[derive(Debug, Clone, Insertable )]
#[diesel(table_name=rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Queryable)]
pub struct Crate {
    pub id : i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime
}


#[derive(Debug, Clone, Insertable)]
#[diesel(table_name=crates)]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}