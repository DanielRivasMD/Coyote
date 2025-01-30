////////////////////////////////////////////////////////////////////////////////////////////////////

// library wrapper
use coyote::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use std::io;
use unicode_normalization::UnicodeNormalization;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::custom::cards::Flashcard;

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> anyResult<()> {
  let mut flashcard = Flashcard::new();
  // hardcoded for debugging
  flashcard.id = "a".nfc().collect::<String>();

  loop {
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read");

		answer = answer.trim().nfc().collect::<String>();
    println!("You answered: {}", answer);

    if flashcard.id == answer {
      if flashcard.quality < 5 {
        flashcard.quality += 1;
      }
    } else {
      println!("wrong!");
      if flashcard.quality > 0 {
        flashcard.quality -= 1;
      }
    }

    flashcard.update();
    println!(
      "Quality: {}, Repetitions: {}, Interval: {} days, Ease Factor: {:.2}",
      flashcard.quality, flashcard.repetitions, flashcard.interval, flashcard.ease_factor
    );

    if flashcard.quality == 5 {
      break
    }
  }

	Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
