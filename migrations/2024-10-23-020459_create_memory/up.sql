----------------------------------------------------------------------------------------------------
-- database architecture
----------------------------------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS memory (
  id              INTEGER PRIMARY KEY AUTOINCREMENT,
  lang            TEXT,
  item            TEXT,
  example         TEXT,
  kind            TEXT,
  quality         TEXT,
  difficulty      TEXT,
  repetitions     TEXT,
  interval        TEXT,
  interval_days   INTEGER NOT NULL DEFAULT 0,
  class           TEXT,
  level           TEXT
);

----------------------------------------------------------------------------------------------------
