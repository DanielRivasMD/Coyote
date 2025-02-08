
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use colored::*;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use anyhow::{
  Context,
  Result as anyResult,
};
use rand::{
  rng,
  seq::SliceRandom,
};
use std::io;
use diesel::SqliteConnection;

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

#[rustfmt::skip]
pub fn train_cli(conn: &mut SqliteConnection, lang: String) -> anyResult<()> {
// retrieve from database
    let mut cards = get_memory(conn, lang)?;

    // create random number generator
    let mut rng = rng();

    // shuffle the array
    cards.shuffle(&mut rng);

    // enable raw mode
    terminal::enable_raw_mode()?;
    // BUG: indentation

    // iterate on data
    for mut card in cards {
      println!("Do you remember this item: {}", card.item.red());
      println!("{}", card.example.cyan());
      println!("Press Left if you remember, Right if you do not.");

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
              println!("Next time");
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

      // update scores
      card.update_score(conn)?;
      println!(
        "Quality: {}, Repetitions: {}, Interval: {}, Ease Factor: {:.2}",
        card.quality, card.repetitions, card.interval, card.difficulty
      );
    }

      // disable raw mode
      terminal::disable_raw_mode()?;

    Ok(())
    }

////////////////////////////////////////////////////////////////////////////////////////////////////
