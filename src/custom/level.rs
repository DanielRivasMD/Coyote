////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum Level {
  A1,
  A2,
  B1,
  B2,
  C1,
  C2,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Default for Level {
  fn default() -> Self {
    Level::A1
  }
}

impl fmt::Display for Level {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Level::A1 => write!(f, "A1"),
      Level::A2 => write!(f, "A2"),
      Level::B1 => write!(f, "B1"),
      Level::B2 => write!(f, "B2"),
      Level::C1 => write!(f, "C1"),
      Level::C2 => write!(f, "C2"),
    }
  }
}

impl TryFrom<&str> for Level {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "A1" => Ok(Level::A1),
      "A2" => Ok(Level::A2),
      "B1" => Ok(Level::B1),
      "B2" => Ok(Level::B2),
      "C1" => Ok(Level::C1),
      "C2" => Ok(Level::C2),
      _ => Err(format!("Invalid level: {}", value)),
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl FromSql<Text, Sqlite> for Level {
  fn from_sql(bytes: SqliteValue) -> diesel::deserialize::Result<Self> {
    let t = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    Ok(t.as_str().try_into()?)
  }
}

impl ToSql<Text, Sqlite> for Level {
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
    out.set_value(self.to_string());
    Ok(diesel::serialize::IsNull::No)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
