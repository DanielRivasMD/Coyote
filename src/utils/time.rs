// standard libraries
use anyhow::{Context, Result as anyResult};
use chrono::{Duration, NaiveDate, Utc};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::CoyoteError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::DATE_FORMAT;

////////////////////////////////////////////////////////////////////////////////////////////////////

// get current date
pub fn current_date() -> String {
  let now = Utc::now().naive_utc();
  now.format(DATE_FORMAT).to_string()
}

// add delta date
pub fn delta_date(
  date_str: String,
  days: String,
) -> anyResult<String> {
  let days = days.parse::<i64>().context(CoyoteError::Parsing { f: days })?;
  let delta = NaiveDate::parse_from_str(&date_str, DATE_FORMAT)?
    .checked_add_signed(Duration::days(days))
    .context(CoyoteError::DateParsing { f: date_str })?
    .to_string();
  Ok(delta)
}

// calculate time difference
pub fn diff_date(
  today: String,
  delta: String,
) -> anyResult<f64> {
  let today = NaiveDate::parse_from_str(&today, DATE_FORMAT)?;
  let delta = NaiveDate::parse_from_str(&delta, DATE_FORMAT)?;
  let diff = today.signed_duration_since(delta);
  Ok(diff.num_days() as f64)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
