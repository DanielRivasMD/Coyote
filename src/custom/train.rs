////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use std::io;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  custom::cards::Card,
  utils::sql::*,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn train() -> anyResult<()> {
  let cards = get_memory()?;
  for card in cards {
    println!("{}", card.item);
    println!("{}", card.example);

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read");

    answer = answer.trim().to_string();
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
