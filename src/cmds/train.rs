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
use crate::{
  custom::cards::FieldsToUpdate,
  utils::sql::*,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn train() -> anyResult<()> {
  // set connection
  let conn = &mut set_conn_db()?;

  // retrieve from database
  let cards = get_memory(conn)?;

  // iterate on data
  for mut card in cards {
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

    if card.item == answer {
      println!("correct!");
      if card.quality.parse::<u32>().unwrap() < 5 {
        card.set_field(
          conn,
          FieldsToUpdate::Quality,
          card.quality.parse::<u32>().unwrap(),
          1,
          |v, f| v + f,
        );
      }
    } else {
      println!("wrong!");
      if card.quality.parse::<u32>().unwrap() > 0 {
        card.set_field(
          conn,
          FieldsToUpdate::Quality,
          card.quality.parse::<u32>().unwrap(),
          1,
          |v, f| v - f,
        );
      }
    }

    // card.update();
    println!(
      "Quality: {}, Repetitions: {}, Interval: {} days, Ease Factor: {:.2}",
      card.quality, card.repetitions, card.interval, card.difficulty
    );
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
