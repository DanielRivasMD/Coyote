////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use diesel::prelude::*;
use serde::{
  Deserialize,
  Serialize,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  custom::schema::memory as memory_table,
  daedalus,
  utils::traits::StringLoader,
};
use crate::custom::schema::memory::dsl::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(new, Debug, Default, Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = memory_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Card {
  #[new(default)]
  pub item: String,

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

impl Card {
  fn set_difficulty(&mut self, conn: &mut SqliteConnection, v: f64) {
    diesel::update(memory.filter(item.eq(self.item.clone())))
      .set(difficulty.eq(v.to_string()))
      .returning(Card::as_returning())
      .get_result(conn)
      .unwrap();
  }

  fn set_interval(&mut self, conn: &mut SqliteConnection, v: u32) {
    diesel::update(memory.filter(item.eq(self.item.clone())))
      .set(interval.eq(v.to_string()))
      .returning(Card::as_returning())
      .get_result(conn)
      .unwrap();
  }

  fn set_repetitions(&mut self, conn: &mut SqliteConnection, v: u32) {
    diesel::update(memory.filter(item.eq(self.item.clone())))
      .set(repetitions.eq(v.to_string()))
      .returning(Card::as_returning())
      .get_result(conn)
      .unwrap();
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Card {
  pub fn update_quality<F>(&mut self, conn: &mut SqliteConnection, q: u32, i: u32, f: F) where F: Fn(u32, u32) -> u32 {
    diesel::update(memory.filter(item.eq(self.item.clone())))
      .set(quality.eq(f(q, i).to_string()))
      .returning(Card::as_returning())
      .get_result(conn)
      .unwrap();
  }
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
    self.update_item(fields[0].into())?;
    self.update_misc(fields[1].into())?;
    self.update_example(fields[2].into())?;
    Ok(())
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// automatic interactive method implementation through daedalus
daedalus!(
  pub, Card;
  item; get_item_owned, get_item_ref, update_item - String, &str
  example; get_example_owned, get_example_ref, update_example - String, &str
  misc; get_misc_owned, get_misc_ref, update_misc - String, &str
);

////////////////////////////////////////////////////////////////////////////////////////////////////
