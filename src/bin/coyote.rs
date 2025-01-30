////////////////////////////////////////////////////////////////////////////////////////////////////

// library wrapper
use coyote::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use std::io;
// use unicode_normalization::UnicodeNormalization;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::custom::cards::Flashcard;

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> anyResult<()> {
  let mut flashcard = Flashcard::new(1);

  flashcard.id = 7;
  // flashcard.id = "a".nfc().collect::<String>();

  // // example usage: simulate user responses with different qualities
  // let qualities = vec![5, 4, 3, 2, 5, 5];

  loop {
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read");

		let answer: u32 = match answer.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Please type a number!");
				continue
			},
		};

		// answer = answer.nfc().collect::<String>();

      println!("You answered: {}", answer);
      println!("{:?}", flashcard);
      println!("{}", flashcard.id == answer);

      if flashcard.id == answer {
        if flashcard.quality < 5 {
          flashcard.quality += 1;
          break;
        } else {
          if flashcard.quality > 0 {
            flashcard.quality -= 1;
          }
        }
      }

    flashcard.update();
    println!(
      "Repetitions: {}, Interval: {} days, Ease Factor: {:.2}",
      flashcard.repetitions, flashcard.interval, flashcard.ease_factor
    );
  }

	Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
