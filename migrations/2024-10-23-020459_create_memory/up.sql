-- Your SQL goes here
CREATE TABLE `memory`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`word` TEXT,
	`kind` TEXT,
	`score` INTEGER,
	`stability` INTEGER,
	`retrievability` INTEGER,
	`difficulty` INTEGER
);

