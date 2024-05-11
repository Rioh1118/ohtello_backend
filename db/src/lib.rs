pub mod schema;
pub mod models;
pub mod error;
pub mod user;
pub mod game;
pub mod r#move;
pub mod history;



use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::error::DbError;


pub fn establish_connection() -> Result<PgConnection, DbError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let con = PgConnection::establish(&database_url)?;
    Ok(con)
}

