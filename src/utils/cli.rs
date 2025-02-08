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
  rng,
  seq::SliceRandom,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::CoyoteError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
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
      print!("\nDo you remember this item: {}\n{}\nPress Enter if you remember, Space if you do not.\n", card.item.red(), card.example.cyan());

      // enable raw mode
      terminal::enable_raw_mode()?;

      if event::poll(std::time::Duration::from_secs(60))? {
        if let Event::Key(event) = event::read()? {
          match event.code {
            KeyCode::Enter => {
              println!("Superb!");
              if card.quality.parse::<u32>().context(CoyoteError::Parsing { f: card.quality.clone(), })? < 5 {
                card.set_field(conn,Fields::Quality,card.quality.parse::<u32>().context(CoyoteError::Parsing { f: card.quality.clone(), })?, 1, |v, f| v + f, )?;
              }
            }

            KeyCode::Char(' ') => {
              println!("Do not give up");
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
