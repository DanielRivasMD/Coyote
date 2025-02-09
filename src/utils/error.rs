////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use colored::*;
use std::path::PathBuf;
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Error)]
pub enum CoyoteError {
  #[error("\n{}: {f:?}\n", "Fail to create file".red())]
  CreateFile { f: PathBuf },

  #[error("\n{}: {f:?}\n", "Fail to read file".red())]
  ReadFile { f: PathBuf },

  #[error("\n{}: {f:?}\n", "Fail to read external script".red())]
  ReadExternalScript { f: String },

  #[error("\n{}: {f:?}\n", "Fail to write file".red())]
  WriteFile { f: String },

  #[error("\n{}: {f:?}\n", "Fail to parse string".red())]
  Parsing { f: String },

  #[error("\n{}: {f:?}\n", "Fail to parse flag".red())]
  ParseFlag { f: String },

  #[error("\n{}\n", "Fail to read line".red())]
  RegistryLine,

  #[error("\n{}: {f:?}\n", "Error connecting to database".red())]
  DatabaseConnection { f: String },

  #[error("\n{}: {f:?}\n", "Error with database address from `env`".red())]
  DatabaseEnv { f: String },

  #[error("\n{}\n", "Error loading data".red())]
  DatabaseLoad { f: String },

  #[error("\n{}\n", "Error updating data".red())]
  DatabaseUpdate { f: String },

  #[error("\n{}\n", "Error parsing date".red())]
  DateParsing { f: String },
}

////////////////////////////////////////////////////////////////////////////////////////////////////
