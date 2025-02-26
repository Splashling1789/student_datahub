-- Your SQL goes here

-- First I remove the previous UNIQUE from subjects(short_name).
PRAGMA foreign_keys=off;

CREATE TABLE subjects_new (
                              id  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                              period_id    INTEGER NOT NULL,
                              short_name    TEXT NOT NULL,
                              name  TEXT NOT NULL,
                              final_score  REAL,
                              FOREIGN KEY (period_id) REFERENCES periods
);

INSERT INTO subjects_new (id, period_id, short_name, name, final_score)
SELECT id, period_id, short_name, name, final_score FROM subjects;
DROP TABLE subjects;
ALTER TABLE subjects_new RENAME TO subjects;
PRAGMA foreign_keys=on;
-- Now I add the UNIQUE constraint to (short_name, period_id)
CREATE UNIQUE INDEX idx_period_id_short_name ON subjects (period_id, short_name);
