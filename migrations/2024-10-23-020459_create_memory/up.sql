----------------------------------------------------------------------------------------------------
-- database architecture
----------------------------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS memory (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  lang TEXT,
  item TEXT,
  example TEXT,
  kind TEXT,
  quality TEXT,
  difficulty TEXT,
  repetitions TEXT,
  interval TEXT,
  class TEXT,
  level TEXT
);

----------------------------------------------------------------------------------------------------
