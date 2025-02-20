////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use diesel::SqliteConnection;
use std::path::PathBuf;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::utils::traits::StringLoader;
use crate::{
  custom::{cards::Card, language::Language},
  utils::{
    io::byte_read_io,
    sql::{insert_struct, set_conn_db},
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn load(
  input: &PathBuf,
  lang: String,
) -> anyResult<()> {
  // open database connection
  let conn = set_conn_db()?;

  // read input
  read_load(conn, input.to_path_buf(), Language::try_from(lang).unwrap())?;

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn read_load(
  mut conn: SqliteConnection,
  file: PathBuf,
  lang: Language,
) -> anyResult<()> {
  // read input
  let mut lines = byte_read_io(file)?;

  // iterate on lines
  while let Some(line) = lines.next() {
    // read line
    let line_read = String::from_utf8_lossy(line?);
    let fields = line_read.split(',').collect::<Vec<&str>>();

    // load from line
    let mut card = Card::load_from_str(fields.clone())?;

    // load from argument
    card.lang = lang.clone();

    // insert to database
    insert_struct(card, &mut conn)?;
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
