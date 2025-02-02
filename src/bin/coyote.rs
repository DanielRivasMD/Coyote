////////////////////////////////////////////////////////////////////////////////////////////////////

// library wrapper
use coyote::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use clap::Parser;
use std::io;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::utils::reader::*;
use crate::utils::help::*;
use crate::utils::sql::*;
use crate::custom::cards::Card;

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> anyResult<()> {

  // collect command line arguments
      let params = Cli::parse();

      match &params.command {
          Commands::Load { input } => {

              // open database connection
              let conn = establish_db_connection()?;

              reader(input.to_path_buf(), conn)?;
          }
          Commands::Train {  } => {
          let mut flashcard = Card::new();
          // hardcoded for debugging
          flashcard.word = "a".to_string();

          loop {
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).expect("Failed to read");

            answer = answer.trim().to_string();
            println!("You answered: {}", answer);

            if flashcard.word == answer {
              if flashcard.quality.parse::<f64>().unwrap() < 5. {
                flashcard.quality = (flashcard.quality.parse::<f64>().unwrap() + 1.).to_string();
              }
            } else {
              println!("wrong!");
              if flashcard.quality.parse::<f64>().unwrap() > 0. {
                flashcard.quality = ((flashcard.quality.parse::<f64>().unwrap() - 1.)).to_string();
              }
            }

            flashcard.update();
            println!(
              "Quality: {}, Repetitions: {}, Interval: {} days, Ease Factor: {:.2}",
              flashcard.quality, flashcard.repetitions, flashcard.interval, flashcard.difficulty
            );

            if flashcard.quality.parse::<f64>().unwrap() == 5. {
              break
            }
          }
          }
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
