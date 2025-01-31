////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
// use anyhow::Result as anyResult;
use diesel::prelude::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::custom::schema::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, Insertable)]
#[diesel(table_name = memory)]
pub struct Card {
  pub word: String,
  pub quality: String,
  pub difficulty: String,
  pub interval: String,
  pub repetitions: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Flashcard {
  pub fn new() -> Self {
    Flashcard {
      id: String::new(),
      quality: 0,       // initialize at zero
      ease_factor: 2.5, // default ease factor
      interval: 1,      // default interval
      repetitions: 0,   // no repetitions initially
    }
  }

  pub fn update(&mut self) {
    // ensure quality is in the range [0, 5]
    // let quality = quality.min(5).max(0);

    if self.quality >= 3 {
      if self.repetitions == 0 {
        self.interval = 1;
      } else if self.repetitions == 1 {
        self.interval = 6;
      } else {
        self.interval = (self.interval as f64 * self.ease_factor).round() as u32;
      }
      self.repetitions += 1;
    } else {
      self.repetitions = 0;
      self.interval = 1;
    }

    // update ease factor
    self.ease_factor += 0.1 - (5 - self.quality) as f64 * (0.08 + (5 - self.quality) as f64 * 0.02);
    if self.ease_factor < 1.3 {
      self.ease_factor = 1.3;
    }
  }

  fn next_review_in_days(&self) -> u32 {
    self.interval
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
