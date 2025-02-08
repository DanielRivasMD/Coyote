////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::{
  Context,
  Result as anyResult,
};
use colored::*;
use crossterm::{
  event::{
    self,
    Event,
    KeyCode,
  },
  terminal::{
    self,
  },
};
use diesel::SqliteConnection;
use rand::{
  Rng,
  rng,
  seq::SliceRandom,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::CoyoteError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  TRAIN_FAILURE,
  TRAIN_SUCCESS,
  custom::fields::Fields,
  utils::sql::get_memory,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[rustfmt::skip]
pub fn train_cli(conn: &mut SqliteConnection, lang: String) -> anyResult<()> {
// retrieve from database
    let mut cards = get_memory(conn, lang)?;

    // create random number generator
    let mut rng = rng();

    // shuffle the array
    cards.shuffle(&mut rng);

    // iterate on data
    for mut card in cards {
      print!("\nDo you remember: {}\nHere is an example: {}\nPress {} if you remember, {} if you do not, or {} to exti\n",
        card.item.bright_blue(), card.example.bright_yellow(), "ENTER".bright_white(), "SPACE".bright_white(), "Q".bright_white()
      );

      // enable raw mode
      terminal::enable_raw_mode()?;

      if event::poll(std::time::Duration::from_secs(60))? {
        if let Event::Key(event) = event::read()? {
          match event.code {
            KeyCode::Enter => {
              print!("\n{}\n", TRAIN_SUCCESS[rng.random_range(0..TRAIN_SUCCESS.len())].bright_green());
              if card.quality.parse::<u32>().context(CoyoteError::Parsing { f: card.quality.clone(), })? < 5 {
                card.set_field(conn,Fields::Quality,card.quality.parse::<u32>().context(CoyoteError::Parsing { f: card.quality.clone(), })?, 1, |v, f| v + f, )?;
              }
            }

            KeyCode::Char(' ') => {
              print!("\n{}\n", TRAIN_FAILURE[rng.random_range(0..TRAIN_FAILURE.len())].bright_red());
              if card.quality.parse::<u32>().context(CoyoteError::Parsing { f: card.quality.clone(), })? > 0 {
                card.set_field(conn,Fields::Quality,card.quality.parse::<u32>().context(CoyoteError::Parsing { f: card.quality.clone(), })?,1,|v, f| v - f, )?;
              }
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

    Ok(())
    }

////////////////////////////////////////////////////////////////////////////////////////////////////
