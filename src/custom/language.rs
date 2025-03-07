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

#[derive(Clone, Debug, AsExpression, FromSqlRow, Deserialize, Serialize, EnumIter)]
#[diesel(sql_type = Text)]
pub enum Language {
  English,
  Espanol,
  Francais,
  Italiano,
  Norsk,
  Test,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Default for Language {
  fn default() -> Self {
    Language::English
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl fmt::Display for Language {
  fn fmt(
    &self,
    f: &mut fmt::Formatter<'_>,
  ) -> fmt::Result {
    match self {
      Language::English => write!(f, "English"),
      Language::Espanol => write!(f, "Espanol"),
      Language::Francais => write!(f, "Francais"),
      Language::Italiano => write!(f, "Italiano"),
      Language::Norsk => write!(f, "Norsk"),
      Language::Test => write!(f, "Test"),
    }
  }
}

impl TryFrom<&str> for Language {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "en" | "EN" | "english" | "English" => Ok(Language::English),
      "es" | "ES" | "espanol" | "Espanol" => Ok(Language::Espanol),
      "fr" | "FR" | "francais" | "Francais" => Ok(Language::Francais),
      "it" | "IT" | "italiano" | "Italiano" => Ok(Language::Italiano),
      "no" | "NO" | "norsk" | "Norsk" => Ok(Language::Norsk),
      "ts" | "TS" | "test" | "Test" => Ok(Language::Test),
      _ => Err(format!("Invalid language: {}", value)),
    }
  }
}

impl TryFrom<String> for Language {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    match value.as_str() {
      "en" | "EN" | "english" | "English" => Ok(Language::English),
      "es" | "ES" | "espanol" | "Espanol" => Ok(Language::Espanol),
      "fr" | "FR" | "francais" | "Francais" => Ok(Language::Francais),
      "it" | "IT" | "italiano" | "Italiano" => Ok(Language::Italiano),
      "no" | "NO" | "norsk" | "Norsk" => Ok(Language::Norsk),
      "ts" | "TS" | "test" | "Test" => Ok(Language::Test),
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
