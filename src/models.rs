use std::io::Write;
use std::str::FromStr;
use std::fmt;
use chrono::NaiveDateTime;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::deserialize::{FromSql, FromSqlRow,self};
use diesel::pg::{Pg, PgValue};
use diesel::{expression::AsExpression, prelude::*};
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use crate::schema::{rustaceans, crates, users_roles, users, roles}; 

#[derive(Debug, Clone, Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Crate {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = crates)]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Queryable, AsChangeset, Serialize, Deserialize, Identifiable)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    pub password: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Queryable, AsChangeset, Serialize, Deserialize, Identifiable)]
pub struct Role {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub code: RoleCode,
    pub name: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub code: RoleCode,
    pub name: String,
}

#[derive(Debug, Clone, Queryable, Deserialize, Associations, Identifiable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name = users_roles)]  
pub struct UserRole {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = users_roles)]  
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}


#[derive(AsExpression, Debug, Clone, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type=Text)]
pub enum RoleCode {
    Admin,
    Editor,
    Viewer,
}

/// Implement Display trait instead of ToString for better integration with Rust's formatting system
impl fmt::Display for RoleCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            RoleCode::Admin => "admin",
            RoleCode::Editor => "editor",
            RoleCode::Viewer => "viewer",
        })
    }
}

// Implement FromStr trait for converting strings to RoleCode
impl FromStr for RoleCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(RoleCode::Admin),
            "editor" => Ok(RoleCode::Editor),
            "viewer" => Ok(RoleCode::Viewer),
            _ => Err(()),
        }
    }
}

// Implement FromSql trait for deserializing RoleCode from SQL
impl FromSql<Text, Pg> for RoleCode {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"admin" => Ok(RoleCode::Admin),
            b"editor" => Ok(RoleCode::Editor),
            b"viewer" => Ok(RoleCode::Viewer),
            _ => Ok(RoleCode::Viewer)
        }
    }
}

// Implement ToSql trait for serializing RoleCode to SQL
impl ToSql<Text, Pg> for RoleCode {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            RoleCode::Admin => out.write_all(b"admin")?,
            RoleCode::Editor => out.write_all(b"editor")?,
            RoleCode::Viewer => out.write_all(b"viewer")?,
        }
        Ok(IsNull::No)
    }
}