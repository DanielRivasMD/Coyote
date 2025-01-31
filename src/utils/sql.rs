////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::custom::cards::*;
use crate::custom::schema::memory::dsl::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn establish_db_connection() -> anyResult<SqliteConnection> {
    let db_path = get_db_path()?.clone();

    Ok(SqliteConnection::establish(db_path.as_str())
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_path)))
}

pub fn insert_insertable_struct(
    card: Card,
    conn: &mut SqliteConnection,
) -> anyResult<()> {
    insert_into(memory)
        .values(&card)
        .execute(conn)?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// DOC: hardcoded variable
// TODO: cli arg?
fn get_db_path() -> anyResult<String> {
    Ok("coyote.db".to_string())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
