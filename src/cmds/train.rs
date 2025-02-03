////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::{
  Context,
  Result as anyResult,
};
use std::io;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::CoyoteError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::utils::sql::get_memory;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn train() -> anyResult<()> {
  // retrieve from database
  let cards = get_memory()?;

  // iterate on data
  for card in cards {
    println!("{}", card.item);
    println!("{}", card.example);

    // capture user input
    let mut answer = String::new();
    io::stdin()
      .read_line(&mut answer)
      .context(CoyoteError::RegistryLine)?;
    answer = answer.trim().to_string();

    // display answers
    println!("You answered: {}", answer);
    println!("{}", answer == card.item);

    // TODO: update values & upload on database
    // if card.item == answer {
    //   if card.quality.parse::<f64>().unwrap() < 5. {
    //     card.quality = (card.quality.parse::<f64>().unwrap() +
    // 1.).to_string();   }
    // } else {
    //   println!("wrong!");
    //   if card.quality.parse::<f64>().unwrap() > 0. {
    //     card.quality = (card.quality.parse::<f64>().unwrap() -
    // 1.).to_string();   }
    // }

    // card.update();
    // println!(
    //   "Quality: {}, Repetitions: {}, Interval: {} days, Ease Factor: {:.2}",
    //   card.quality, card.repetitions, card.interval, card.difficulty
    // );
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
