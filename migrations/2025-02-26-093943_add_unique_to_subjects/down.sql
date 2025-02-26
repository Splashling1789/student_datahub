-- WARNING: Subjects with duplicate short names will be ignored.
PRAGMA foreign_keys=off;
CREATE TABLE subjects_old (
                              id  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                              period_id    INTEGER NOT NULL,
                              short_name    TEXT NOT NULL UNIQUE,
                              name  TEXT NOT NULL,
                              final_score  REAL,
                              FOREIGN KEY (period_id) REFERENCES periods
);

INSERT OR IGNORE INTO subjects_old (id, period_id, short_name, name, final_score)
SELECT id, period_id, short_name, name, final_score FROM subjects;
DROP TABLE subjects;
ALTER TABLE subjects_old RENAME TO subjects;

PRAGMA foreign_keys=on;
DROP INDEX IF EXISTS idx_period_id_short_name;