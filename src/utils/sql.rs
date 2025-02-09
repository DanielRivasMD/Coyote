////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::{Context, Result as anyResult};
use diesel::{insert_into, prelude::*, sqlite::SqliteConnection};
use dotenvy::dotenv;
use std::env;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::CoyoteError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::custom::{cards::*, schema::memory::dsl::*};

use super::time::current_date;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn set_conn_db() -> anyResult<SqliteConnection> {
  dotenv().ok();
  let db_path =
    env::var("DATABASE_URL").context(CoyoteError::DatabaseEnv { f: "DATABASE_URL".to_string() })?;

  Ok(
    SqliteConnection::establish(db_path.as_str())
      .context(CoyoteError::DatabaseConnection { f: db_path })?,
  )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn insert_struct(
  card: Card,
  conn: &mut SqliteConnection,
) -> anyResult<()> {
  insert_into(memory).values(&card).execute(conn)?;
  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// retrieve all records from database
pub fn get_memory(
  conn: &mut SqliteConnection,
  filter_lang: String,
) -> anyResult<Vec<Card>> {
  let results: Vec<Card> = memory
    .filter(lang.eq(filter_lang.clone()))
    .filter(interval.lt(current_date()))
    // .select((item, example, misc, quality, difficulty, interval, repetitions))
    .select(Card::as_select())
    .load::<Card>(conn)
    .context(CoyoteError::DatabaseLoad { f: filter_lang.clone() })?;

  Ok(results)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
