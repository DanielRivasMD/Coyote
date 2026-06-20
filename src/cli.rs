////////////////////////////////////////////////////////////////////////////////////////////////////

use clap::{Parser, Subcommand};
use std::path::PathBuf;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::custom::log;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Logging level
    #[arg(short, long, value_enum, default_value_t = log::LogFlag::Info)]
    pub log: log::LogFlag,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Subcommand)]
pub enum Command {
    /// Read input from user for training
    Read {
        /// Language to use (optional)
        #[arg(long)]
        lang: Option<String>,
    },

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
    },
}

////////////////////////////////////////////////////////////////////////////////////////////////////
