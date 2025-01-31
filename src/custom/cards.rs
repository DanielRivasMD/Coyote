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

impl Card {
  pub fn new() -> Self {
    Card {
      word: String::new(),
      quality: "0".to_string(),                      // initialize at zero
      difficulty: "2.5".to_string(),                 // default ease factor
      interval: "1".to_string(),                     // default interval
      repetitions: "0".to_string(),                  // no repetitions initially
    }
  }

  pub fn update(&mut self) {

    // quality is locked between 0 - 5
    if self.quality.parse::<u32>().unwrap() >= 3 {
      if self.repetitions.parse::<u32>().unwrap() == 0 {
        self.interval = "1".to_string();
      } else if self.repetitions.parse::<u32>().unwrap() == 1 {
        self.interval = "6".to_string();
      } else {
        self.interval = ((self.interval.parse::<f64>().unwrap() * self.difficulty.parse::<f64>().unwrap()).round() as u32).to_string();
      }
      self.repetitions = (self.repetitions.parse::<u32>().unwrap() + 1).to_string();
    } else {
      self.repetitions = "0".to_string();
      self.interval = "1".to_string();
    }

    // update difficulty
    self.difficulty = (self.difficulty.parse::<f64>().unwrap() + 0.1 - (5. - self.quality.parse::<f64>().unwrap()) * (0.08 + (5. - self.quality.parse::<f64>().unwrap()) * 0.02)).to_string();
    if self.difficulty.parse::<f64>().unwrap() < 1.3 {
      self.difficulty = "1.3".to_string();
    }
  }

  // fn next_review_in_days(&self) -> u32 {
  //   self.interval
  // }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
