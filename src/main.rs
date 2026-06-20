////////////////////////////////////////////////////////////////////////////////////////////////////

// library wrapper
use coyote::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use clap::Parser;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::cmd::diag::diag;
use crate::cmd::load::load;
use crate::cmd::read::read;
use crate::cmd::train::train;
use crate::util::help::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> anyResult<()> {
    // collect command line arguments
    let params = Cli::parse();

    // match sub commands
    match &params.command {
        Commands::Read { lang } => {
            read(lang.clone())?;
        }

        Commands::Load { input, lang } => {
            load(input, lang.clone())?;
        }

        Commands::Train { lang } => {
            train(lang.clone())?;
        }

        Commands::Diag { lang } => {
            diag(lang.clone())?;
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
