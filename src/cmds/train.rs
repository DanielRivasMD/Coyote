////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::{
  Context,
  Result as anyResult,
};
use rand::{
  rng,
  seq::SliceRandom,
};
use std::io;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::CoyoteError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  custom::fields::Fields,
  utils::sql::{
    get_memory,
    set_conn_db,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: set filter from date onwards
// TODO: relocate train logic
pub fn train(lang: String) -> anyResult<()> {
  // set connection
  let conn = &mut set_conn_db()?;

  // retrieve from database
  let mut cards = get_memory(conn, lang)?;

  // create random number generator
  let mut rng = rng();

  // shuffle the array
  cards.shuffle(&mut rng);

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
      if card.quality.parse::<u32>().context(CoyoteError::Parsing {
        f: card.quality.clone(),
      })? <
        5
      {
        card.set_field(
          conn,
          Fields::Quality,
          card.quality.parse::<u32>().context(CoyoteError::Parsing {
            f: card.quality.clone(),
          })?,
          1,
          |v, f| v + f,
        )?;
      }
    } else {
      println!("wrong!");
      if card.quality.parse::<u32>().context(CoyoteError::Parsing {
        f: card.quality.clone(),
      })? >
        0
      {
        card.set_field(
          conn,
          Fields::Quality,
          card.quality.parse::<u32>().context(CoyoteError::Parsing {
            f: card.quality.clone(),
          })?,
          1,
          |v, f| v - f,
        )?;
      }
    }

    // update scores
    card.update_score(conn)?;
    println!(
      "Quality: {}, Repetitions: {}, Interval: {}, Ease Factor: {:.2}",
      card.quality, card.repetitions, card.interval, card.difficulty
    );
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
