CREATE TABLE IF NOT EXISTS activity
(
    source            TEXT PRIMARY KEY,
    title             TEXT NOT NULL,
    category          TEXT,
    date              TEXT,
    duration_in_hours INT,
    description       TEXT,
    audiences         TEXT,
    organizer         TEXT NOT NULL
)