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

  #[error("\n{}: {f:?}\n", "Fail to parse".red())]
  Parsing { f: String },

  #[error("\n{}: {f:?}\n", "Fail to parse flag".red())]
  ParseFlag { f: String },

  #[error("\n{}\n", "Fail to read lines".red())]
  RegistryLine,

  #[error("\n{}: {f:?}\n", "Could not load input".red())]
  Loading { f: String },
}

////////////////////////////////////////////////////////////////////////////////////////////////////
