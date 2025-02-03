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

pub fn establish_db_connection() -> anyResult<SqliteConnection> {
  dotenv().ok();
  let db_path = env::var("DATABASE_URL").context(CoyoteError::DatabaseEnv {
    f: "DATABASE_URL".to_string(),
  })?;

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
  let conn = &mut establish_db_connection()?;
  let results: Vec<Card> = memory
    // .filter(difficulty.eq("2.5"))
    // .select((item, example, misc, quality, difficulty, interval, repetitions))
    .select(Card::as_select())
    .load::<Card>(conn)
    .context(CoyoteError::DatabaseLoad)?;

  Ok(results)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
