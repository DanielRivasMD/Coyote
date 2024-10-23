CREATE TABLE memory (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  token TEXT,
  kind TEXT,
  score INTEGER,
  stability INTEGER,
  retrievability INTEGER,
  difficulty INTEGER
);
