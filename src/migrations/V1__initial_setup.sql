CREATE TABLE IF NOT EXISTS activity (
                        source      TEXT PRIMARY KEY,
                        title       TEXT NOT NULL,
                        category    TEXT,
                        date        TEXT,
                        duration    TEXT,
                        description TEXT,
                        audiences   TEXT,
                        organizer   TEXT
)