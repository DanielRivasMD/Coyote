////////////////////////////////////////////////////////////////////////////////////////////////////

// library wrapper
use coyote::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use clap::Parser;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::utils::help::*;
use crate::cmds::train::train;
use crate::cmds::load::load;
use crate::cmds::read::read;

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> anyResult<()> {
  // collect command line arguments
  let params = Cli::parse();

  // match sub commands
  match &params.command {
    Commands::Read {  } => {
      read()?;
    }

    Commands::Load { input, lang } => {
      load(input, lang.clone())?;
    }

    Commands::Train { lang } => {
      train(lang.clone())?;
    }
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
