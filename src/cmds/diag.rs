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
use crate::custom::{language::Language, level::Level, score::Score};
use crate::utils::sql::set_conn_db;
use crate::{TRAIN_FAILURE, TRAIN_SUCCESS, utils::sql::get_memory};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn diag(lang: String) -> anyResult<()> {
  // set connection
  let conn = &mut set_conn_db()?;

  // diagnose logic
  diagnose(conn, Language::try_from(lang).unwrap())?;

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[rustfmt::skip]
fn diagnose(conn: &mut SqliteConnection, lang: Language) -> anyResult<()> {

  // preallocate scores
  let mut scores = vec![];

  for level in Level::iter() {


    println!("{}", level);

    // initialize score
    let mut level_score = Score::new(level.clone());

    // retrieve from database
    let mut cards = get_memory(conn, &lang.to_string(), &level.to_string())?;

    // create random number generator
    let mut rng = rng();

    // shuffle the array
    cards.shuffle(&mut rng);

    // iterate on data
    for card in cards {
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
              level_score.increase_count()?;
              level_score.increase_total()?;
            }

            KeyCode::Char(' ') => {
              print!("\n{}\n", TRAIN_FAILURE[rng.random_range(0..TRAIN_FAILURE.len())].bright_red());
              level_score.increase_total()?;
            }

            KeyCode::Char('q') => {
              println!("Exiting...");
              level_score.calculate_score()?;
              println!("The score for level: {} is {}", level, level_score.score);
              break;
            }

            _ => {}
          }
        }
      }

      // disable raw mode
      terminal::disable_raw_mode()?;
    }

    // update scores
    level_score.calculate_score()?;
    scores.push((level.to_string(), level_score.score));
    println!("The score for level: {} is {}", level, level_score.score);

    // disable raw mode
    terminal::disable_raw_mode()?;

    // prevent next level
    if scores[scores.len()].1 < 0.75 { break }

  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
