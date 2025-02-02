////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use anyhow::Context;
use bytelines::ByteLines;
use bytelines::ByteLinesReader;
use diesel::SqliteConnection;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::CoyoteError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::utils::sql::insert_insertable_struct;
use crate::custom::cards::Card;
use crate::utils::traits::StringLoader;

////////////////////////////////////////////////////////////////////////////////////////////////////

fn byte_file_reader(input_file: PathBuf) -> anyResult<ByteLines<BufReader<File>>> {
  let file =
    File::open(&input_file).context(CoyoteError::ReadFile {
      f: input_file.into(),
    })?;

  let reader = BufReader::new(file);

  let lines = reader.byte_lines();
  Ok(lines)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn reader(file: PathBuf, mut conn: SqliteConnection) -> anyResult<()> {

  // read input
  let mut lines = byte_file_reader(file)?;

  // iterate on lines
  while let Some(line) = lines.next() {

    // read line
    let line_read = String::from_utf8_lossy(line?);
    let fields = line_read.split(',').collect::<Vec<&str>>();

    // load from line
    let card = Card::load_from_str(fields.clone())?;

    // insert to database
    insert_insertable_struct(card, &mut conn)?;
  }

  Ok(())
}


////////////////////////////////////////////////////////////////////////////////////////////////////
