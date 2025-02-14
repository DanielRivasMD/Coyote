////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use diesel::{
  AsExpression, FromSqlRow,
  deserialize::FromSql,
  serialize::{Output, ToSql},
  sql_types::Text,
  sqlite::{Sqlite, SqliteValue},
};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::EnumIter;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, AsExpression, FromSqlRow, Deserialize, Serialize, EnumIter)]
#[diesel(sql_type = Text)]
pub enum Language {
  Italiano,
  Norsk,
  Francais,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Default for Language {
  fn default() -> Self {
    Language::Italiano
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl fmt::Display for Language {
  fn fmt(
    &self,
    f: &mut fmt::Formatter<'_>,
  ) -> fmt::Result {
    match self {
      Language::Italiano => write!(f, "Italiano"),
      Language::Norsk => write!(f, "Norsk"),
      Language::Francais => write!(f, "Francais"),
    }
  }
}

impl TryFrom<&str> for Language {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "Italiano" => Ok(Language::Italiano),
      "Norsk" => Ok(Language::Norsk),
      "Francais" => Ok(Language::Francais),
      _ => Err(format!("Invalid language: {}", value)),
    }
  }
}

impl TryFrom<String> for Language {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    match value.as_str() {
      "Italiano" => Ok(Language::Italiano),
      "Norsk" => Ok(Language::Norsk),
      "Francais" => Ok(Language::Francais),
      _ => Err(format!("Invalid language: {}", value)),
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl FromSql<Text, Sqlite> for Language {
  fn from_sql(bytes: SqliteValue) -> diesel::deserialize::Result<Self> {
    let t = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    Ok(t.as_str().try_into()?)
  }
}

impl ToSql<Text, Sqlite> for Language {
  fn to_sql<'b>(
    &'b self,
    out: &mut Output<'b, '_, Sqlite>,
  ) -> diesel::serialize::Result {
    out.set_value(self.to_string());
    Ok(diesel::serialize::IsNull::No)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
