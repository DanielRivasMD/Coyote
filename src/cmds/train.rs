////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::{
  Context,
  Result as anyResult,
};
use diesel::prelude::*;
use std::io;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::CoyoteError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::utils::sql::*;
use crate::custom::schema::memory::dsl::*;
use crate::custom::cards::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn train() -> anyResult<()> {
  // set connection
  let conn = &mut set_conn_db()?;

  // retrieve from database
  let cards = get_memory(conn)?;

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

    // TODO: refactor to function
    if card.item == answer {
      println!("correct!");
      if card.quality.parse::<f64>().unwrap() < 5. {
        diesel::update(memory.filter(item.eq(card.item)))
          .set(quality.eq((card.quality.parse::<f64>().unwrap() + 1.).to_string()))
          .returning(Card::as_returning())
          .get_result(conn)
          .unwrap();
      }
    } else {
      println!("wrong!");
      if card.quality.parse::<f64>().unwrap() > 0. {
        diesel::update(memory.filter(item.eq(card.item)))
          .set(quality.eq((card.quality.parse::<f64>().unwrap() - 1.).to_string()))
          .returning(Card::as_returning())
          .get_result(conn)
          .unwrap();
      }
    }

    // card.update();
    // println!(
    //   "Quality: {}, Repetitions: {}, Interval: {} days, Ease Factor: {:.2}",
    //   card.quality, card.repetitions, card.interval, card.difficulty
    // );
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
