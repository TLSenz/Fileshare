
CREATE TABLE file (
                      id INTEGER PRIMARY KEY AUTOINCREMENT, -- SQLite's way of auto-incrementing primary key
                      file_name TEXT NOT NULL,
                      hashed_file_name TEXT NOT NULL UNIQUE,
                      content_hash TEXT NOT NULL,         -- SQLite treats CHAR(64) as TEXT
                      content_type TEXT NOT NULL,
                      size INTEGER NOT NULL,
                      storage_path TEXT NOT NULL,
                      owner_id INTEGER,                   -- SQLite treats INT as INTEGER
                      is_public INTEGER DEFAULT 0,        -- SQLite uses INTEGER (0 or 1) for BOOLEAN
                      is_deleted INTEGER DEFAULT 0,       -- SQLite uses INTEGER (0 or 1) for BOOLEAN
                      created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                      updated_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- ON UPDATE CURRENT_TIMESTAMP needs a trigger in SQLite
                      deleted_at DATETIME NULL,

                      FOREIGN KEY (owner_id) REFERENCES users(id)
                          ON DELETE SET NULL
                          ON UPDATE CASCADE
);