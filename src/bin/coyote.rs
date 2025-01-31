////////////////////////////////////////////////////////////////////////////////////////////////////

// library wrapper
use coyote::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
// use std::io;
// use unicode_normalization::UnicodeNormalization;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> anyResult<()> {


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
