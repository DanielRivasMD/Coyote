////////////////////////////////////////////////////////////////////////////////////////////////////

// library wrapper
use coyote::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use clap::Parser;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::custom::cards::Card;
use crate::utils::reader::*;
use crate::utils::help::*;
use crate::utils::sql::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: split subcommands
fn main() -> anyResult<()> {
  // collect command line arguments
  let params = Cli::parse();

  // open database connection
  let mut conn = establish_db_connection()?;

  // get path
  let file = params.input;

  // read input
  let mut lines = byte_file_reader(file)?;

  // TODO: migrate to function
  // iterate on lines
  while let Some(line) = lines.next() {

    // read line
    let line_read = String::from_utf8_lossy(line?);
    let fields = line_read.split(',').collect::<Vec<&str>>();

    // TODO: migrate to method
    let mut card = Card::new();

    card.word = fields[0].to_string().clone();

    // insert to database
    insert_insertable_struct(card, &mut conn)?;

  }

  // let mut flashcard = Card::new();
  // // hardcoded for debugging
  // flashcard.word = "a".nfc().collect::<String>();

  // loop {
  //   let mut answer = String::new();
  //   io::stdin().read_line(&mut answer).expect("Failed to read");

  //   answer = answer.trim().nfc().collect::<String>();
  //   println!("You answered: {}", answer);

  //   if flashcard.word == answer {
  //     if flashcard.quality < 5 {
  //       flashcard.quality += 1;
  //     }
  //   } else {
  //     println!("wrong!");
  //     if flashcard.quality > 0 {
  //       flashcard.quality -= 1;
  //     }
  //   }

  //   flashcard.update();
  //   println!(
  //     "Quality: {}, Repetitions: {}, Interval: {} days, Ease Factor: {:.2}",
  //     flashcard.quality, flashcard.reps, flashcard.interval, flashcard.difficulty
  //   );

  //   if flashcard.quality == 5 {
  //     break
  //   }
  // }

	Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
