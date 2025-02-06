////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::{
  Context,
  Result as anyResult,
};
use diesel::prelude::*;
use serde::{
  Deserialize,
  Serialize,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  custom::schema::memory::{
    self as memory_table,
    dsl::*,
  },
  daedalus,
  utils::{
    error::CoyoteError,
    traits::StringLoader,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(new, Debug, Default, Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = memory_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Card {
  #[new(default)]
  pub lang: String,

  #[new(default)]
  pub item: String,

  #[new(default)]
  pub example: String,

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
  pub fn update_score(
    &mut self,
    conn: &mut SqliteConnection,
  ) -> anyResult<()> {
    // quality is locked between 0 - 5
    if self.quality.parse::<u32>().context(CoyoteError::Parsing {
      f: self.quality.clone(),
    })? >=
      3
    {
      if self
        .repetitions
        .parse::<u32>()
        .context(CoyoteError::Parsing {
          f: self.repetitions.clone(),
        })? ==
        0
      {
        self.set_field(conn, FieldsToUpdate::Interval, 1, 0, |v, f| v)?;
      } else if self
        .repetitions
        .parse::<u32>()
        .context(CoyoteError::Parsing {
          f: self.repetitions.clone(),
        })? ==
        1
      {
        self.set_field(conn, FieldsToUpdate::Interval, 6, 0, |v, f| v)?;
      } else {
        self.set_field(
          conn,
          FieldsToUpdate::Interval,
          self.interval.parse::<f64>().context(CoyoteError::Parsing {
            f: self.interval.clone(),
          })?,
          self
            .difficulty
            .parse::<f64>()
            .context(CoyoteError::Parsing {
              f: self.difficulty.clone(),
            })?,
          |v, f| ((v * f).round() as u32).into(),
        )?;
      }
      self.set_field(
        conn,
        FieldsToUpdate::Repetitions,
        self
          .repetitions
          .parse::<u32>()
          .context(CoyoteError::Parsing {
            f: self.repetitions.clone(),
          })?,
        1,
        |v, f| v + f,
      )?;
    } else {
      self.set_field(conn, FieldsToUpdate::Repetitions, 0, 0, |v, f| v)?;
      self.set_field(conn, FieldsToUpdate::Interval, 1, 0, |v, f| v)?;
    }

    // update difficulty
    self.set_field(
      conn,
      FieldsToUpdate::Difficulty,
      self
        .difficulty
        .parse::<f64>()
        .context(CoyoteError::Parsing {
          f: self.difficulty.clone(),
        })? +
        0.1 -
        (5. -
          self.quality.parse::<f64>().context(CoyoteError::Parsing {
            f: self.difficulty.clone(),
          })?),
      0.08 +
        (5. -
          self.quality.parse::<f64>().context(CoyoteError::Parsing {
            f: self.quality.clone(),
          })?) *
          0.02,
      |v, f| v * f,
    )?;

    if self
      .difficulty
      .parse::<f64>()
      .context(CoyoteError::Parsing {
        f: self.difficulty.clone(),
      })? <
      1.3
    {
      self.set_field(conn, FieldsToUpdate::Difficulty, 1.3, 0., |v, f| v)?;
    }

    Ok(())
  }

  // fn next_review_in_days(&self) -> u32 {
  //   self.interval
  // }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Card {
  pub fn set_field<T: ToString, F>(
    &mut self,
    conn: &mut SqliteConnection,
    column: FieldsToUpdate,
    value: T,
    factor: T,
    lambda: F,
  ) -> anyResult<()>
  where
    F: Fn(T, T) -> T,
  {
    match column {
      FieldsToUpdate::Quality => {
        diesel::update(memory.filter(item.eq(self.item.clone())))
          .set(quality.eq(lambda(value, factor).to_string()))
          .returning(Card::as_returning())
          .get_result(conn)
          .context(CoyoteError::DatabaseUpdate)?;
      }
      FieldsToUpdate::Difficulty => {
        diesel::update(memory.filter(item.eq(self.item.clone())))
          .set(difficulty.eq(value.to_string()))
          .returning(Card::as_returning())
          .get_result::<Card>(conn)
          .context(CoyoteError::DatabaseUpdate)?;
      }
      FieldsToUpdate::Interval => {
        diesel::update(memory.filter(item.eq(self.item.clone())))
          .set(interval.eq(value.to_string()))
          .returning(Card::as_returning())
          .get_result::<Card>(conn)
          .context(CoyoteError::DatabaseUpdate)?;
      }
      FieldsToUpdate::Repetitions => {
        diesel::update(memory.filter(item.eq(self.item.clone())))
          .set(repetitions.eq(value.to_string()))
          .returning(Card::as_returning())
          .get_result::<Card>(conn)
          .context(CoyoteError::DatabaseUpdate)?;
      }
    };

    Ok(())
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
    self.update_example(fields[1].into())?;
    Ok(())
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// automatic interactive method implementation through daedalus
daedalus!(
  pub, Card;
  item; get_item_owned, get_item_ref, update_item - String, &str
  example; get_example_owned, get_example_ref, update_example - String, &str
);

////////////////////////////////////////////////////////////////////////////////////////////////////
