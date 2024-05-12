pub mod error;
pub mod game;
pub mod history;
pub mod models;
pub mod r#move;
pub mod schema;
pub mod user;

use crate::error::DbError;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> Result<PgConnection, DbError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let con = PgConnection::establish(&database_url)?;
    Ok(con)
}
