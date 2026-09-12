CREATE TABLE IF NOT EXISTS groups (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '',
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS items (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    group_id      INTEGER NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    item_type     TEXT NOT NULL CHECK (item_type IN ('command', 'script')),
    name          TEXT NOT NULL CHECK (length(trim(name)) > 0), -- content for commands
    content       TEXT NOT NULL DEFAULT '' CHECK (item_type != 'script' OR length(trim(content)) > 0),
    description   TEXT NOT NULL DEFAULT '',
    is_favourite  INTEGER NOT NULL DEFAULT 0 CHECK (is_favourite IN (0, 1)),
    last_used_at  TEXT,
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(group_id, name)
);

CREATE TABLE IF NOT EXISTS tags (
    item_id  INTEGER NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    name     TEXT NOT NULL,
    PRIMARY KEY (item_id, name)
);

-- All commands/scripts in a group
CREATE INDEX IF NOT EXISTS idx_item_group_id ON items(group_id);

-- Items by type
CREATE INDEX IF NOT EXISTS idx_item_type ON items(item_type);

-- Favorites lists
CREATE INDEX IF NOT EXISTS idx_item_favourite ON items(is_favourite) WHERE is_favourite = 1;

-- Recently used
CREATE INDEX IF NOT EXISTS idx_item_last_used ON items(last_used_at DESC);

-- Tag search by name
CREATE INDEX IF NOT EXISTS idx_tag_name ON tags(name);
