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

pub enum FieldsToUpdate {
  Quality,
  Difficulty,
  Interval,
  Repetitions,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Card {
  pub fn update_score(&mut self, conn: &mut SqliteConnection) {
    // quality is locked between 0 - 5
    if self.quality.parse::<u32>().unwrap() >= 3 {
      if self.repetitions.parse::<u32>().unwrap() == 0 {
        self.set_interval(conn, 1);
      } else if self.repetitions.parse::<u32>().unwrap() == 1 {
        self.set_interval(conn, 6);
      } else {
        self.set_interval(conn, (self.interval.parse::<f64>().unwrap() *
                  self.difficulty.parse::<f64>().unwrap())
                .round() as u32);
      }
      self.set_repetitions(conn, self.repetitions.parse::<u32>().unwrap() + 1);
    } else {
      self.set_repetitions(conn, 0);
      self.set_interval(conn, 1);
    }

    // update difficulty
    self.set_difficulty(conn, self.difficulty.parse::<f64>().unwrap() + 0.1 -
      (5. - self.quality.parse::<f64>().unwrap()) *
        (0.08 + (5. - self.quality.parse::<f64>().unwrap()) * 0.02));
    if self.difficulty.parse::<f64>().unwrap() < 1.3 {
      self.set_difficulty(conn, 1.3);
    }
  }

  // fn next_review_in_days(&self) -> u32 {
  //   self.interval
  // }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Card {
    pub fn set_field<T: ToString, F>(&mut self, conn: &mut SqliteConnection, column: FieldsToUpdate, value: T, factor: T, lambda: F) where F: Fn(T, T) -> T {

match column {
  FieldsToUpdate::Quality => {
  diesel::update(memory.filter(item.eq(self.item.clone())))
    .set(quality.eq(lambda(value, factor).to_string()))
    .returning(Card::as_returning())
    .get_result(conn)
    .unwrap();
  },
  FieldsToUpdate::Difficulty => {
              diesel::update(memory.filter(item.eq(self.item.clone())))
                .set(difficulty.eq(value.to_string()))
                .returning(Card::as_returning())
                .get_result::<Card>(conn)
                .unwrap();
            },
            FieldsToUpdate::Interval => {
              diesel::update(memory.filter(item.eq(self.item.clone())))
                .set(interval.eq(value.to_string()))
                .returning(Card::as_returning())
                .get_result::<Card>(conn)
                .unwrap();
            },
            FieldsToUpdate::Repetitions => {
              diesel::update(memory.filter(item.eq(self.item.clone())))
                .set(repetitions.eq(value.to_string()))
                .returning(Card::as_returning())
                .get_result::<Card>(conn)
                .unwrap();
            },
        };
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
