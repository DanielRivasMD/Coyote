////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use clap::{Parser, Subcommand};
use std::path::PathBuf;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::custom::log_flag::LogFlag;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,

  /// Logging level
  #[arg(short, long, value_enum, default_value_t = LogFlag::Info)]
  pub log: LogFlag,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Subcommand)]
pub enum Commands {
  /// Read input from user for training
  Read {},

  /// Load data for training from file
  Load {
    /// Input file
    #[arg(long)]
    input: PathBuf,

    /// Language to load
    #[arg(long)]
    lang: String,
  },

  /// Train your skills
  Train {
    /// Language to train
    #[arg(long)]
    lang: String,
  },

  /// Diagnose your skill level
  Diag {
    /// Language to diagnose
    #[arg(long)]
    lang: String,
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
