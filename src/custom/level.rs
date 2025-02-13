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

