////////////////////////////////////////////////////////////////////////////////////////////////////

use anyhow::{Context, Result as anyResult};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::util::error::CoyoteError;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::custom::fields::Fields;
use crate::custom::language::Language;
use crate::custom::level::Level;
use crate::custom::schema::memory::{self as memory_table, dsl::*};
use crate::daedalus;
use crate::util::time::diff_date;
use crate::util::{
    time::{current_date, delta_date},
    traits::StringLoader,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(new, Debug, Default, Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = memory_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Card {
    #[new(default)]
    pub id: i32,

    #[new(default)]
    pub lang: Language,

    #[new(default)]
    pub item: String,

    #[new(default)]
    pub example: String,

    #[new(default)]
    pub kind: String,

    #[new(value = "String::from(\"0\")")]
    pub quality: String,

    #[new(value = "String::from(\"2.5\")")]
    pub difficulty: String,

    #[new(value = "String::from(\"0\")")]
    pub repetitions: String,

    #[new(default)]
    pub interval: String,

    #[new(default)]
    pub interval_days: i32,

    #[new(default)]
    pub class: String,

    #[new(default)]
    pub level: Level,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Card {
    pub fn update_score(&mut self, conn: &mut SqliteConnection) -> anyResult<()> {
        // Parse numeric fields once
        let q: u32 = self.quality.parse().context(CoyoteError::Parsing {
            f: self.quality.clone(),
        })?;
        let mut ef: f64 = self.difficulty.parse().context(CoyoteError::Parsing {
            f: self.difficulty.clone(),
        })?;
        let rep: u32 = self.repetitions.parse().context(CoyoteError::Parsing {
            f: self.repetitions.clone(),
        })?;
        let cur_interval: i32 = self.interval_days;

        let new_rep: u32;
        let new_interval: i32;

        if q < 3 {
            // Reset – do NOT change ease factor
            new_rep = 0;
            new_interval = 1;
        } else {
            new_rep = rep + 1;
            new_interval = match rep {
                0 => 1,
                1 => 6,
                _ => (cur_interval as f64 * ef).round() as i32,
            };

            // Update ease factor ONLY on successful recall
            let q_f64 = q as f64;
            ef = ef + (0.1 - (5.0 - q_f64) * (0.08 + (5.0 - q_f64) * 0.02));
            if ef < 1.3 {
                ef = 1.3;
            }
        }

        let today = current_date();
        let next_review = delta_date(today, new_interval.to_string())?;

        // Persist changes using id
        diesel::update(memory.filter(id.eq(self.id)))
            .set((
                quality.eq(q.to_string()),
                difficulty.eq(ef.to_string()),
                repetitions.eq(new_rep.to_string()),
                interval.eq(next_review.clone()),
                interval_days.eq(new_interval),
            ))
            .execute(conn)
            .context(CoyoteError::DatabaseUpdate {
                f: self.item.clone(),
            })?;

        // Keep in‑memory state consistent
        self.quality = q.to_string();
        self.difficulty = ef.to_string();
        self.repetitions = new_rep.to_string();
        self.interval = next_review;
        self.interval_days = new_interval;

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
                let updated = diesel::update(memory.filter(id.eq(self.id)))
                    .set(quality.eq(lambda(value, factor).to_string()))
                    .returning(Card::as_returning())
                    .get_result(conn)
                    .context(CoyoteError::DatabaseUpdate {
                        f: self.item.clone(),
                    })?;
                *self = updated;
            }
            Fields::Difficulty => {
                let updated = diesel::update(memory.filter(id.eq(self.id)))
                    .set(difficulty.eq(lambda(value, factor).to_string()))
                    .returning(Card::as_returning())
                    .get_result::<Card>(conn)
                    .context(CoyoteError::DatabaseUpdate {
                        f: self.item.clone(),
                    })?;
                *self = updated;
            }
            Fields::Interval => {
                let updated = diesel::update(memory.filter(id.eq(self.id)))
                    .set(interval.eq(delta_date(
                        current_date(),
                        lambda(value, factor).to_string(),
                    )?))
                    .returning(Card::as_returning())
                    .get_result::<Card>(conn)
                    .context(CoyoteError::DatabaseUpdate {
                        f: self.item.clone(),
                    })?;
                *self = updated;
            }
            Fields::Repetitions => {
                let updated = diesel::update(memory.filter(id.eq(self.id)))
                    .set(repetitions.eq(lambda(value, factor).to_string()))
                    .returning(Card::as_returning())
                    .get_result::<Card>(conn)
                    .context(CoyoteError::DatabaseUpdate {
                        f: self.item.clone(),
                    })?;
                *self = updated;
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

    fn update_from_str(&mut self, fields: Vec<&str>) -> anyResult<()> {
        // update fields
        self.update_item(fields[0].into())?;
        self.update_example(fields[1].into())?;
        self.update_class(fields[2].into())?;
        self.update_level(fields[3].into())?;
        Ok(())
    }
}

// implement update level manually
impl Card {
    pub fn update_level(&mut self, flines: &str) -> anyResult<()> {
        self.level = Level::try_from(flines).unwrap();
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// automatic interactive method implementation through daedalus
daedalus!(
  pub, Card;
  item; get_item_owned, get_item_ref, update_item - String, &str
  example; get_example_owned, get_example_ref, update_example - String, &str
  class; get_class_owned, get_class_ref, update_class - String, &str
);

////////////////////////////////////////////////////////////////////////////////////////////////////
