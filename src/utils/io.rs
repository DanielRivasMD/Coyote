////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::{
  Context,
  Result as anyResult,
};
use bytelines::{
  ByteLines,
  ByteLinesReader,
};
use diesel::SqliteConnection;
use std::{
  fs::File,
  io::BufReader,
  path::PathBuf,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::CoyoteError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  custom::cards::Card,
  utils::{
    sql::insert_struct,
    traits::StringLoader,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

fn byte_read_io(input_file: PathBuf) -> anyResult<ByteLines<BufReader<File>>> {
  let file = File::open(&input_file).context(CoyoteError::ReadFile {
    f: input_file.into()
  })?;

  let reader = BufReader::new(file);

  let lines = reader.byte_lines();
  Ok(lines)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn read_io(
  mut conn: SqliteConnection,
  file: PathBuf,
  lang: String,
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
