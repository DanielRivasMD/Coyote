////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::{Context, Result as anyResult};
use bytelines::{ByteLines, ByteLinesReader};
use std::{fs::File, io::BufReader, path::PathBuf};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::CoyoteError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn byte_read_io(input_file: PathBuf) -> anyResult<ByteLines<BufReader<File>>> {
  let file = File::open(&input_file).context(CoyoteError::ReadFile { f: input_file.into() })?;
  let reader = BufReader::new(file);
  let lines = reader.byte_lines();
  Ok(lines)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
