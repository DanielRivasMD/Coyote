////////////////////////////////////////////////////////////////////////////////////////////////////

use anyhow::Result as anyResult;
use diesel::SqliteConnection;
use std::path::PathBuf;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::custom::cards::Card;
use crate::custom::language::Language;
use crate::util::io;
use crate::util::sql;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn run(input: &PathBuf, lang: String) -> anyResult<()> {
    let conn = sql::set_conn_db()?;
    read_load(conn, input.to_path_buf(), Language::try_from(lang).unwrap())?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn read_load(mut conn: SqliteConnection, file: PathBuf, lang: Language) -> anyResult<()> {
    let mut lines = io::byte_read_io(file)?;

    // Read the header line
    let header_line = match lines.next() {
        Some(Ok(line)) => String::from_utf8_lossy(line).to_string(),
        Some(Err(e)) => return Err(e.into()),
        None => return Ok(()), // empty file
    };

    let headers: Vec<&str> = header_line.split(',').map(|h| h.trim()).collect();
    let expected_len = headers.len();

    // Determine positions of known columns
    let pos_item = headers.iter().position(|&h| h == "item");
    let pos_example = headers.iter().position(|&h| h == "example");
    let pos_class = headers.iter().position(|&h| h == "class");
    let pos_level = headers.iter().position(|&h| h == "level");
    let pos_lang = headers.iter().position(|&h| h == "lang");

    // Item and example are mandatory
    if pos_item.is_none() || pos_example.is_none() {
        anyhow::bail!("CSV header must contain 'item' and 'example' columns");
    }

    let pos_item = pos_item.unwrap();
    let pos_example = pos_example.unwrap();
    let pos_class = pos_class;
    let pos_level = pos_level;
    let pos_lang = pos_lang;

    // Process data rows with a manual line counter
    let mut line_num = 0;
    while let Some(line_result) = lines.next() {
        line_num += 1;
        let line = line_result?;
        let line_str = String::from_utf8_lossy(line);

        if line_str.trim().is_empty() {
            continue;
        }

        let fields: Vec<&str> = line_str.split(',').collect();

        if fields.len() != expected_len {
            eprintln!(
                "Warning: line {} has {} fields (expected {}), skipping: {}",
                line_num + 1, // +1 because line_num counts after header
                fields.len(),
                expected_len,
                line_str
            );
            continue;
        }

        // Extract required fields
        let item = fields[pos_item].to_string();
        let example = fields[pos_example].to_string();

        // Build card from extracted fields
        let mut card = Card::new();
        card.update_item(item)?;
        card.update_example(example)?;

        if let Some(p) = pos_class {
            card.update_class(fields[p].to_string())?;
        }
        if let Some(p) = pos_level {
            card.update_level(fields[p])?;
        }

        // Language: prefer CSV column, fallback to CLI argument
        if let Some(p) = pos_lang {
            card.lang = Language::try_from(fields[p]).map_err(|e| anyhow::anyhow!("{}", e))?;
        } else {
            card.lang = lang.clone();
        }

        println!("{:?}", card);
        sql::insert_struct(card, &mut conn)?;
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
