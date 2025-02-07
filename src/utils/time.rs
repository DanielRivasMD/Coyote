
// standard libraries
use anyhow::{
  Context,
  Result as anyResult,
};
use chrono::{NaiveDateTime, Utc, Duration};

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
    now.format("%Y-%m-%d").to_string()
}

// add delta date
pub fn delta_date(date_str: String, days: String) -> anyResult<String> {
  let days = days.parse::<i64>().unwrap();
  let delta = NaiveDateTime::parse_from_str(&date_str, DATE_FORMAT)?.checked_add_signed(Duration::days(days)).context(CoyoteError::DateParsing)?.to_string();
  Ok(delta)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
