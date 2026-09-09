CREATE TABLE IF NOT EXISTS groups (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '',
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS commands (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    group_id      INTEGER NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    content       TEXT NOT NULL CHECK (length(trim(content)) > 0),
    description   TEXT NOT NULL DEFAULT '',
    is_favourite  INTEGER NOT NULL DEFAULT 0 CHECK (is_favourite IN (0, 1)),
    last_used_at  TEXT,
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(group_id, content)
);

CREATE TABLE IF NOT EXISTS scripts (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    group_id      INTEGER NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    name          TEXT NOT NULL CHECK (length(trim(name)) > 0),
    content       TEXT NOT NULL DEFAULT '',
    description   TEXT NOT NULL DEFAULT '',
    is_favourite  INTEGER NOT NULL DEFAULT 0 CHECK (is_favourite IN (0, 1)),
    last_used_at  TEXT,
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(group_id, name)
);

CREATE TABLE command_tags (
    command_id INTEGER NOT NULL REFERENCES commands(id) ON DELETE CASCADE,
    name       TEXT NOT NULL,
    PRIMARY KEY (command_id, name)
);

CREATE TABLE script_tags (
    script_id INTEGER NOT NULL REFERENCES scripts(id) ON DELETE CASCADE,
    name      TEXT NOT NULL,
    PRIMARY KEY (script_id, name)
);

-- All commands/scripts in a group
CREATE INDEX IF NOT EXISTS idx_command_group_id ON commands(group_id);
CREATE INDEX IF NOT EXISTS idx_script_group_id  ON scripts(group_id);

-- Favorites lists
CREATE INDEX IF NOT EXISTS idx_command_favourite ON commands(is_favourite) WHERE is_favourite = 1;
CREATE INDEX IF NOT EXISTS idx_script_favourite  ON scripts(is_favourite)  WHERE is_favourite = 1;

-- Recently used
CREATE INDEX IF NOT EXISTS idx_command_last_used ON commands(last_used_at DESC);
CREATE INDEX IF NOT EXISTS idx_script_last_used  ON scripts(last_used_at DESC);

-- Tag search by name
CREATE INDEX IF NOT EXISTS idx_command_tag_name ON command_tags(name);
CREATE INDEX IF NOT EXISTS idx_script_tag_name  ON script_tags(name);
