////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod diagnose;
pub mod load;
pub mod read;
pub mod train;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod completion {

    use anyhow::Result as anyResult;
    use clap::{Command, CommandFactory};
    use clap_complete::{generate, shells::*};
    use std::io;

    use crate::cli;

    pub fn run(shell: cli::Shell) -> anyResult<()> {
        let visible: Vec<_> = cli::Cli::command()
            .get_subcommands()
            .filter(|s| !s.is_hide_set())
            .cloned()
            .collect();

        let mut cmd = Command::new(env!("CARGO_PKG_NAME")).subcommands(visible);

        let name = cmd.get_name().to_string();

        match shell {
            cli::Shell::Bash => generate(Bash, &mut cmd, name, &mut io::stdout()),
            cli::Shell::Zsh => generate(Zsh, &mut cmd, name, &mut io::stdout()),
            cli::Shell::Fish => generate(Fish, &mut cmd, name, &mut io::stdout()),
            cli::Shell::Powershell => generate(PowerShell, &mut cmd, name, &mut io::stdout()),
        }
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod identity {
    use anyhow::Result as anyResult;

    const IDENTITY: &str = r#"In Greek mythology, Cerberus, Κέρβερος, often referred to as the hound of Hades, is a multi-headed dog
that guards the gates of the underworld to prevent the dead from leaving.

He was the offspring of the monsters Echidna and Typhon, and was usually described as having three heads,
a serpent for a tail, and snakes protruding from his body.

Cerberus is primarily known for his capture by Heracles, the last of Heracles' twelve labours"#;

    pub fn run() -> anyResult<()> {
        println!("{}", IDENTITY);
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
