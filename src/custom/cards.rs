////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::{Context, Result as anyResult};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::CoyoteError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use super::language::Language;
use super::level::Level;
use crate::utils::time::diff_date;
use crate::{
  custom::{
    fields::Fields,
    schema::memory::{self as memory_table, dsl::*},
  },
  daedalus,
  utils::{
    time::{current_date, delta_date},
    traits::StringLoader,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(new, Debug, Default, Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = memory_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Card {
  #[new(default)]
  pub lang: Language,

  #[new(default)]
  pub item: String,

  #[new(default)]
  pub example: String,

  #[new(value = "String::from(\"0\")")]
  pub quality: String,

  #[new(value = "String::from(\"2.5\")")]
  pub difficulty: String,

  #[new(value = "String::from(\"0\")")]
  pub repetitions: String,

  #[new(default)]
  pub interval: String,

  #[new(default)]
  pub level: Level,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[rustfmt::skip]
impl Card {
  // TODO: review update logic carefully & document it
  pub fn update_score(
    &mut self,
    conn: &mut SqliteConnection,
  ) -> anyResult<()> {
    // quality is locked between 0 - 5
    if self.quality.parse::<u32>().context(CoyoteError::Parsing { f: self.quality.clone(), })? >= 3 {
      if self.repetitions.parse::<u32>().context(CoyoteError::Parsing { f: self.repetitions.clone(), })? == 0 {
        self.set_field(conn, Fields::Interval, 1, 0, |v, f| v)?;
      } else if self.repetitions.parse::<u32>().context(CoyoteError::Parsing { f: self.repetitions.clone(), })? == 1 {
        self.set_field(conn, Fields::Interval, 6, 0, |v, f| v)?;
      } else {
        self.set_field(conn,Fields::Interval, diff_date(current_date(), self.interval.clone())?, self.difficulty.parse::<f64>().context(CoyoteError::Parsing { f: self.difficulty.clone(), })?, |v, f| ((v * f).round() as u32).into())?;
      }
      self.set_field(conn,Fields::Repetitions,self.repetitions.parse::<u32>().context(CoyoteError::Parsing { f: self.repetitions.clone(), })?, 1, |v, f| v + f, )?;
    } else {
      self.set_field(conn, Fields::Repetitions, 0, 0, |v, f| v)?;
      self.set_field(conn, Fields::Interval, 1, 0, |v, f| v)?;
    }

    // update difficulty
    self.set_field(conn,Fields::Difficulty,self.difficulty.parse::<f64>().context(CoyoteError::Parsing { f: self.difficulty.clone(), })? + 0.1 - (5. - self.quality.parse::<f64>().context(CoyoteError::Parsing { f: self.difficulty.clone(), })?),0.08 + (5. - self.quality.parse::<f64>().context(CoyoteError::Parsing { f: self.quality.clone(), })?) * 0.02, |v, f| v * f)?;
    if self.difficulty.parse::<f64>().context(CoyoteError::Parsing { f: self.difficulty.clone(), })? < 1.3 {
      self.set_field(conn, Fields::Difficulty, 1.3, 0., |v, f| v)?;
    }

    Ok(())
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Card {
  pub fn set_field<T: ToString, F>(
    &mut self,
    conn: &mut SqliteConnection,
    column: Fields,
    value: T,
    factor: T,
    lambda: F,
  ) -> anyResult<()>
  where
    F: Fn(T, T) -> T,
  {
    match column {
      Fields::Quality => {
        diesel::update(memory.filter(item.eq(self.item.clone())))
          .set(quality.eq(lambda(value, factor).to_string()))
          .returning(Card::as_returning())
          .get_result(conn)
          .context(CoyoteError::DatabaseUpdate { f: self.item.clone() })?;
      }
      Fields::Difficulty => {
        diesel::update(memory.filter(item.eq(self.item.clone())))
          .set(difficulty.eq(lambda(value, factor).to_string()))
          .returning(Card::as_returning())
          .get_result::<Card>(conn)
          .context(CoyoteError::DatabaseUpdate { f: self.item.clone() })?;
      }
      Fields::Interval => {
        diesel::update(memory.filter(item.eq(self.item.clone())))
          .set(interval.eq(delta_date(current_date(), lambda(value, factor).to_string())?))
          .returning(Card::as_returning())
          .get_result::<Card>(conn)
          .context(CoyoteError::DatabaseUpdate { f: self.item.clone() })?;
      }
      Fields::Repetitions => {
        diesel::update(memory.filter(item.eq(self.item.clone())))
          .set(repetitions.eq(lambda(value, factor).to_string()))
          .returning(Card::as_returning())
          .get_result::<Card>(conn)
          .context(CoyoteError::DatabaseUpdate { f: self.item.clone() })?;
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
