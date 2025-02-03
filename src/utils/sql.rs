////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::{
  Context,
  Result as anyResult,
};
use diesel::{
  insert_into,
  prelude::*,
  sqlite::SqliteConnection,
};
use dotenvy::dotenv;
use std::env;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::CoyoteError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::custom::{
  cards::*,
  schema::memory::dsl::*,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// DOC: hardcoded variable
// TODO: use env variable
fn get_db_path() -> anyResult<String> {
  Ok("coyote.db".to_string())
}

pub fn establish_db_connection() -> anyResult<SqliteConnection> {
  let db_path = get_db_path()?.clone();
  // dotenv().ok();
  // let database_url = env::var("DATABASE_URL")
  // .expect("DATABASE_URL must be set");

  Ok(
    SqliteConnection::establish(db_path.as_str()).context(CoyoteError::DatabaseConnection {
      f: db_path,
    })?,
  )
}

pub fn insert_insertable_struct(
  card: Card,
  conn: &mut SqliteConnection,
) -> anyResult<()> {
  insert_into(memory).values(&card).execute(conn)?;
  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// retrieve all records from database
pub fn get_memory() -> anyResult<Vec<Card>> {
  let conn = &mut establish_db_connection().unwrap();
  let results: Vec<Card> = memory
    // .filter(difficulty.eq("2.5"))
    // .select((item, example, misc, quality, difficulty, interval, repetitions))
    .select(Card::as_select())
    .load::<Card>(conn)
    .expect("Error loading users");

  Ok(results)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
