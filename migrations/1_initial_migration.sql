-- Groups
CREATE TABLE IF NOT EXISTS groups (
    id          TEXT PRIMARY KEY,          -- e.g. "recent", "favorite", or UUID
    name        TEXT NOT NULL,
    description TEXT,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Items (commands / scripts)
CREATE TABLE IF NOT EXISTS items (
    id            TEXT PRIMARY KEY,        -- UUID
    group_id      TEXT NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    name          TEXT NOT NULL,
    description   TEXT NOT NULL DEFAULT '',
    content_type  TEXT NOT NULL CHECK(content_type IN ('command', 'script')),
    content       TEXT NOT NULL,                    -- the actual command or script body
    tags          TEXT NOT NULL DEFAULT '[]',       -- JSON array
    favourited    INTEGER NOT NULL DEFAULT 0,       -- 0/1
    last_used_at  TEXT,
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_items_group_id ON items(group_id);
CREATE INDEX IF NOT EXISTS idx_items_favourited ON items(favourited);
CREATE INDEX IF NOT EXISTS idx_items_last_used ON items(last_used_at);
