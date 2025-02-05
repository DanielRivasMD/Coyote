////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::{
  Context,
  Result as anyResult,
};
use std::io::{
  Read,
  stdin,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::utils::error::CoyoteError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  custom::cards::Card,
  utils::sql::{
    insert_struct,
    set_conn_db,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn read() -> anyResult<()> {
  // open database connection
  let mut conn = set_conn_db()?;

  // initialize card
  let mut card = Card::new();

  // initialize holders
  let mut item = String::new();
  let mut example = String::new();

  println!("item?");
  stdin()
    .read_line(&mut item)
    .context(CoyoteError::RegistryLine)?;
  card.item = item.trim().to_string();

  println!("example?");
  stdin()
    .read_to_string(&mut example)
    .context(CoyoteError::RegistryLine)?;
  card.example = example;

  // insert to database
  insert_struct(card, &mut conn)?;

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
