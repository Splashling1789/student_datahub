-- Your SQL goes here
CREATE TABLE periods (
    id  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    initial_date    DATE NOT NULL,
    final_date   DATE NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE subjects (
    id  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    period_id    INTEGER NOT NULL,
    short_name    TEXT NOT NULL UNIQUE,
    name  TEXT NOT NULL,
    final_score  REAL,
    FOREIGN KEY (period_id) REFERENCES periods
);

CREATE TABLE entry (
    id  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    date   DATE NOT NULL,
    subject_id   INTEGER NOT NULL,
    dedicated_time INTEGER NOT NULL,
    FOREIGN KEY (subject_id) REFERENCES subjects
)

