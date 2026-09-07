CREATE TABLE IF NOT EXISTS `group` (
    `id`          INTEGER PRIMARY KEY AUTOINCREMENT,
    `name`        TEXT NOT NULL UNIQUE,
    `description` TEXT,
    `created_at`  TEXT NOT NULL DEFAULT (datetime('now')),
    `updated_at`  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS `item` (
    `id`            INTEGER PRIMARY KEY AUTOINCREMENT,
    `group_id`      INTEGER NOT NULL REFERENCES `group`(`id`) ON DELETE CASCADE,
    `name`          TEXT NOT NULL,
    `description`   TEXT,
    `content_type`  TEXT NOT NULL CHECK (`content_type` IN ('command', 'script')), -- 'command' or 'script'
    `content`       TEXT NOT NULL DEFAULT "",
    `favourited`    INTEGER NOT NULL DEFAULT 0,       -- 0 or 1
    `last_used_at`  TEXT,
    `created_at`    TEXT NOT NULL DEFAULT (datetime('now')),
    `updated_at`    TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(`group_id`, `name`)
);

CREATE TABLE IF NOT EXISTS `tag` (
    `item_id`     INTEGER NOT NULL REFERENCES `item`(`id`) ON DELETE CASCADE,
    `name`        TEXT NOT NULL,
    PRIMARY KEY (`item_id`, `name`)
);

CREATE INDEX IF NOT EXISTS idx_items_group_id ON `item`(`group_id`);
CREATE INDEX IF NOT EXISTS idx_items_favourited ON `item`(`favourited`);
CREATE INDEX IF NOT EXISTS idx_items_last_used ON `item`(`last_used_at`);

CREATE INDEX IF NOT EXISTS idx_tags_item_id ON `tag`(`item_id`);
CREATE INDEX IF NOT EXISTS idx_tags_name ON `tag`(`name`);
