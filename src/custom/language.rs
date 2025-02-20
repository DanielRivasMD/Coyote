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
    }
  }
}

impl TryFrom<&str> for Language {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "English" => Ok(Language::English),
      "Espanol" => Ok(Language::Espanol),
      "Francais" => Ok(Language::Francais),
      "Italiano" => Ok(Language::Italiano),
      "Norsk" => Ok(Language::Norsk),
      _ => Err(format!("Invalid language: {}", value)),
    }
  }
}

impl TryFrom<String> for Language {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    match value.as_str() {
      "English" => Ok(Language::English),
      "Espanol" => Ok(Language::Espanol),
      "Francais" => Ok(Language::Francais),
      "Italiano" => Ok(Language::Italiano),
      "Norsk" => Ok(Language::Norsk),
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
