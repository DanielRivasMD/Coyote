////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::custom::level::Level;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(new)]
pub struct Score {
  pub level: Level,

  #[new(default)]
  pub count: u32,

  #[new(default)]
  pub total: u32,

  #[new(default)]
  pub score: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
