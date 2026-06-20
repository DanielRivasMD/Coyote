////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::{Context, Result as anyResult};
use std::io::{self, Write, stdin, stdout};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::util::error::CoyoteError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::custom::cards::Card;
use crate::custom::language::Language;
use crate::custom::level::Level;
use crate::util::sql::{insert_struct, set_conn_db};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn read(lang: Option<String>) -> anyResult<()> {
    let mut conn = set_conn_db()?;
    let mut card = Card::new();

    // Determine default language from argument or fallback to English
    let default_lang = if let Some(ref l) = lang {
        Language::try_from(l.as_str()).map_err(|e| anyhow::anyhow!("{}", e))?
    } else {
        Language::English
    };

    let mut input = String::new();

    // item
    print!("item? ");
    stdout().flush()?;
    stdin()
        .read_line(&mut input)
        .context(CoyoteError::RegistryLine)?;
    card.item = input.trim().to_string();
    input.clear();

    // example
    print!("example? ");
    stdout().flush()?;
    stdin()
        .read_line(&mut input)
        .context(CoyoteError::RegistryLine)?;
    card.example = input.trim().to_string();
    input.clear();

    // class (optional, default empty)
    print!("class (optional, press Enter to skip): ");
    stdout().flush()?;
    stdin()
        .read_line(&mut input)
        .context(CoyoteError::RegistryLine)?;
    let class = input.trim().to_string();
    if !class.is_empty() {
        card.class = class;
    }
    input.clear();

    // level (default A1)
    print!("level [A1, A2, B1, B2, C1, C2] (default A1): ");
    stdout().flush()?;
    stdin()
        .read_line(&mut input)
        .context(CoyoteError::RegistryLine)?;
    let level_str = input.trim();
    card.level = if level_str.is_empty() {
        Level::A1
    } else {
        Level::try_from(level_str).map_err(|e| anyhow::anyhow!("{}", e))?
    };
    input.clear();

    // language (default from CLI or English)
    print!(
        "language (en, es, fr, it, no, ts) [default: {}]: ",
        default_lang.to_string()
    );
    stdout().flush()?;
    stdin()
        .read_line(&mut input)
        .context(CoyoteError::RegistryLine)?;
    let lang_str = input.trim();
    card.lang = if lang_str.is_empty() {
        default_lang
    } else {
        Language::try_from(lang_str).map_err(|e| anyhow::anyhow!("{}", e))?
    };

    insert_struct(card, &mut conn)?;
    println!("Card added successfully.");
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
