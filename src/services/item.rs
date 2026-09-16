use std::sync::Arc;
use sqlx::{FromRow, SqlitePool};

use super::group::{FAVOURITES_GROUP_ID, RECENT_GROUP_ID};

#[derive(Debug, Clone, FromRow)]
pub struct ItemRow {
    pub id: i32,
    pub group_id: i32,
    pub item_type: String,
    pub name: Option<String>,
    pub content: String,
    pub description: String,
    pub is_favourite: i32,
    pub last_used_at: Option<String>,
    pub created_at: String,
}

const RECENT_ITEMS_LIMIT: i32 = 50;

pub async fn load_items_for_group(
    pool: Arc<SqlitePool>,
    group_id: i32,
    commands: bool,
    scripts: bool,
) -> Result<Vec<(ItemRow, Vec<String>)>, String> {
    // Fetch items for special groups
    if group_id == FAVOURITES_GROUP_ID {
        return load_items_for_favourites(pool.clone(), commands, scripts).await;
    }
    if group_id == RECENT_GROUP_ID {
        return load_items_for_recents(pool.clone(), commands, scripts, RECENT_ITEMS_LIMIT).await;
    }

    // Fetch items
    let rows = sqlx::query_as::<_, ItemRow>(
        r#"
        SELECT id, group_id, item_type, name, content, description, is_favourite, last_used_at, created_at
        FROM items
        WHERE group_id = ?
          AND (
            (? AND item_type = 'command')
            OR (? AND item_type = 'script')
          )
        ORDER BY id DESC
        "#,
    )
    .bind(group_id)
    .bind(commands)
    .bind(scripts)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    // Load tags for each item
    let mut out = Vec::with_capacity(rows.len());

    for row in rows {
        let tags = load_tags(pool.clone(), row.id).await?;
        out.push((row, tags));
    }

    Ok(out)
}

pub async fn load_items_for_favourites(
    pool: Arc<SqlitePool>,
    commands: bool,
    scripts: bool,
) -> Result<Vec<(ItemRow, Vec<String>)>, String> {
    let rows = sqlx::query_as::<_, ItemRow>(
        r#"
        SELECT id, group_id, item_type, name, content, description,
               is_favourite, last_used_at, created_at
        FROM items
        WHERE is_favourite = 1
          AND (
            (? AND item_type = 'command')
            OR (? AND item_type = 'script')
          )
        ORDER BY id DESC
        "#,
    )
    .bind(commands)
    .bind(scripts)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut out = Vec::with_capacity(rows.len());

    for row in rows {
        let tags = load_tags(pool.clone(), row.id).await?;
        out.push((row, tags));
    }

    Ok(out)
}

pub async fn load_items_for_recents(
    pool: Arc<SqlitePool>,
    commands: bool,
    scripts: bool,
    limit: i32,
) -> Result<Vec<(ItemRow, Vec<String>)>, String> {
    let rows = sqlx::query_as::<_, ItemRow>(
        r#"
        SELECT id, group_id, item_type, name, content, description,
               is_favourite, last_used_at, created_at
        FROM items
        WHERE last_used_at IS NOT NULL
          AND (
            (? AND item_type = 'command')
            OR (? AND item_type = 'script')
          )
        ORDER BY last_used_at DESC
        LIMIT ?
        "#,
    )
    .bind(commands)
    .bind(scripts)
    .bind(limit)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut out = Vec::with_capacity(rows.len());

    for row in rows {
        let tags = load_tags(pool.clone(), row.id).await?;
        out.push((row, tags));
    }
    Ok(out)
}

async fn load_tags(pool: Arc<SqlitePool>, item_id: i32) -> Result<Vec<String>, String> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM tags WHERE item_id = ? ORDER BY name",
    )
    .bind(item_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows.into_iter().map(|(n,)| n).collect())
}

pub async fn create_item(
    pool: Arc<SqlitePool>,
    group_id: i32,

    item_type: &str,
    name: Option<String>,
    content: String,
    description: String,
    tags: Vec<String>,
) -> Result<i32, String> {
    let (item_type, name, content, description, tags) = validate_fields(item_type, name, content, description, tags)?;

    // Insert item
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let result = sqlx::query(
        r#"
        INSERT INTO items (group_id, item_type, name, content, description)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(group_id)
    .bind(item_type)
    .bind(name)
    .bind(content)
    .bind(description)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    // Insert tags
    let item_id = result.last_insert_rowid() as i32;

    for tag in tags {
        sqlx::query(
            "INSERT OR IGNORE INTO tags (item_id, name) VALUES (?, ?)",
        )
        .bind(item_id)
        .bind(tag)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(item_id)
}

pub async fn update_item(
    pool: Arc<SqlitePool>,
    id: i32,
    item_type: &str,
    name: Option<String>,
    content: String,
    description: String,
    tags: Vec<String>,
) -> Result<(), String> {
    let (_, name, content, description, tags) = validate_fields(item_type, name, content, description, tags)?;

    // Update item
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    sqlx::query(
        r#"
        UPDATE items
        SET name = ?, content = ?, description = ?
        WHERE id = ?
        "#,
    )
    .bind(name)
    .bind(content)
    .bind(description)
    .bind(id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    // Remove tags
    sqlx::query("DELETE FROM tags WHERE item_id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    // Reinsert tags
    for tag in tags {
        sqlx::query(
            "INSERT OR IGNORE INTO tags (item_id, name) VALUES (?, ?)",
        )
        .bind(id)
        .bind(&tag)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn update_item_favourite(
    pool: Arc<SqlitePool>,
    id: i32,
    favourite: bool,
) -> Result<(), String> {
    sqlx::query("UPDATE items SET is_favourite = ? WHERE id = ?")
        .bind(favourite as i32)
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn update_item_used(pool: Arc<SqlitePool>, id: i32) -> Result<(), String> {
    sqlx::query(
        "UPDATE items SET last_used_at = datetime('now') WHERE id = ?",
    )
    .bind(id)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn delete_item(pool: Arc<SqlitePool>, id: i32) -> Result<(), String> {
    sqlx::query("DELETE FROM items WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn validate_fields(
    item_type: &str,
    name: Option<String>,
    content: String,
    description: String,
    tags: Vec<String>,
) -> Result<(String, Option<String>, String, String, Vec<String>), String> {
    let item_type = item_type.trim().to_string();
    let name = match name {
        Some(n) => {
            let n = n.trim().to_string();
            if n.is_empty() { None } else { Some(n) }
        }
        None => None,
    };
    let content = content.trim().to_string();
    let description = description.trim().to_string();
    let tags = tags.iter().map(|t| t.trim()).filter(|t| !t.is_empty()).map(|t| t.to_string()).collect::<Vec<_>>();

    // Item type
    if item_type != "command" && item_type != "script" {
        return Err("Invalid item type".into());
    }

    // Script name
    if item_type == "script" {
        match &name {
            Some(name) => if name.is_empty() {
                return Err("Name is required".into());
            } else if name.len() > 100 {
                return Err("Name is too long (max 100 chars)".into());
            }
            None => return Err("Name is required".into())
        }
    }

    // Content
    if content.is_empty() {
        return Err("Content is required".into());
    } else if item_type == "command" && content.len() > 500 {
        return Err("Content is too long (max 500 characters: Use script instead)".into());
    }

    // Description
    if description.len() > 1000 {
        return Err("Description is too long (max 1000 characters)".into());
    }

    // Tags
    if tags.len() > 30 {
        return Err("Too many tags (max 30 tags)".into());
    }

    for tag in &tags {
        if tag.len() > 20 {
            return Err(format!("Tag '{}' is too long (max 20 characters)", tag).into());
        }
    }

    Ok((item_type, name, content, description, tags))
}
