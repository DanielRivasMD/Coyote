////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use diesel::prelude::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  custom::schema::*,
  daedalus,
  utils::traits::StringLoader,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, Insertable, new)]
#[diesel(table_name = memory)]
pub struct Card {
  #[new(default)]
  pub word: String,

  #[new(default)]
  pub example: String,

  #[new(default)]
  pub misc: String,

  #[new(value = "String::from(\"0\")")]
  pub quality: String,

  #[new(value = "String::from(\"2.5\")")]
  pub difficulty: String,

  #[new(value = "String::from(\"1\")")]
  pub interval: String,

  #[new(value = "String::from(\"0\")")]
  pub repetitions: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Card {
  pub fn update(&mut self) {
    // quality is locked between 0 - 5
    if self.quality.parse::<u32>().unwrap() >= 3 {
      if self.repetitions.parse::<u32>().unwrap() == 0 {
        self.interval = "1".to_string();
      } else if self.repetitions.parse::<u32>().unwrap() == 1 {
        self.interval = "6".to_string();
      } else {
        self.interval = ((self.interval.parse::<f64>().unwrap() *
          self.difficulty.parse::<f64>().unwrap())
        .round() as u32)
          .to_string();
      }
      self.repetitions = (self.repetitions.parse::<u32>().unwrap() + 1).to_string();
    } else {
      self.repetitions = "0".to_string();
      self.interval = "1".to_string();
    }

    // update difficulty
    self.difficulty = (self.difficulty.parse::<f64>().unwrap() + 0.1 -
      (5. - self.quality.parse::<f64>().unwrap()) *
        (0.08 + (5. - self.quality.parse::<f64>().unwrap()) * 0.02))
      .to_string();
    if self.difficulty.parse::<f64>().unwrap() < 1.3 {
      self.difficulty = "1.3".to_string();
    }
  }

  // fn next_review_in_days(&self) -> u32 {
  //   self.interval
  // }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl StringLoader for Card {
  fn new_wrap() -> Self {
    Self::new()
  }

  fn update_from_str(
    &mut self,
    fields: Vec<&str>,
  ) -> anyResult<()> {
    // update fields
    self.update_word(fields[0].into())?;
    self.update_misc(fields[1].into())?;
    self.update_example(fields[2].into())?;
    Ok(())
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// automatic interactive method implementation through daedalus
daedalus!(
  pub, Card;
  word; get_word_owned, get_word_ref, update_word - String, &str
  example; get_example_owned, get_example_ref, update_example - String, &str
  misc; get_misc_owned, get_misc_ref, update_misc - String, &str
);

////////////////////////////////////////////////////////////////////////////////////////////////////
