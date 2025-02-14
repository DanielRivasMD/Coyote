////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use colored::*;
use crossterm::{
  event::{self, Event, KeyCode},
  terminal::{self},
};
use diesel::SqliteConnection;
use rand::{Rng, rng, seq::SliceRandom};
use strum::IntoEnumIterator;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::custom::level::Level;
use crate::utils::sql::set_conn_db;
use crate::{TRAIN_FAILURE, TRAIN_SUCCESS, utils::sql::get_memory};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn diag(lang: String) -> anyResult<()> {
  // set connection
  let conn = &mut set_conn_db()?;

  // train logic
  diagnose(conn, lang)?;

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[rustfmt::skip]
fn diagnose(conn: &mut SqliteConnection, lang: String) -> anyResult<()> {
  for level in Level::iter() {
    println!("{}", level);

    // retrieve from database
    let mut cards = get_memory(conn, &lang, &level.to_string())?;

    // create random number generator
    let mut rng = rng();

    // shuffle the array
    cards.shuffle(&mut rng);

    // iterate on data
    for mut card in cards {
      print!("\nDo you know:     {}\nHere is an example:  {}\nPress {} if you know, {} if you do not, or {} to exit\n",
        card.item.bright_blue(), card.example.bright_yellow(), "ENTER".bright_white(), "SPACE".bright_white(), "Q".bright_white()
      );

      // enable raw mode
      terminal::enable_raw_mode()?;

      if event::poll(std::time::Duration::from_secs(60))? {
        if let Event::Key(event) = event::read()? {
          match event.code {
            KeyCode::Enter => {
              print!("\n{}\n", TRAIN_SUCCESS[rng.random_range(0..TRAIN_SUCCESS.len())].bright_green());
            }

            KeyCode::Char(' ') => {
              print!("\n{}\n", TRAIN_FAILURE[rng.random_range(0..TRAIN_FAILURE.len())].bright_red());
            }

            KeyCode::Char('q') => {
              println!("Exiting...");
              break;
            }

            _ => {}
          }
        }
      }

      // disable raw mode
      terminal::disable_raw_mode()?;

      // update scores
      card.update_score(conn)?;
      print!(
        "\nQuality: {}, Repetitions: {}, Interval: {}, Ease Factor: {:.2}\n",
        card.quality, card.repetitions, card.interval, card.difficulty
      );
    }

    // disable raw mode
    terminal::disable_raw_mode()?;

  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
