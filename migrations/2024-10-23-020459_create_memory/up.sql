----------------------------------------------------------------------------------------------------
-- database architecture
----------------------------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS memory (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  item TEXT,
  example TEXT,
  misc TEXT,
  kind TEXT,
  quality TEXT,
  difficulty TEXT,
  interval TEXT,
  repetitions TEXT
);

----------------------------------------------------------------------------------------------------
