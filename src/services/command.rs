use std::sync::Arc;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, FromRow)]
pub struct CommandRow {
    pub id: i32,
    pub group_id: i32,
    pub content: String,
    pub description: String,
    pub is_favourite: i32,
    pub last_used_at: Option<String>,
}

pub async fn load_commands_for_group(pool: Arc<SqlitePool>, group_id: i32,) -> Result<Vec<(CommandRow, Vec<String>)>, String> {
    let rows = sqlx::query_as::<_, CommandRow>(
        r#"
        SELECT id, group_id, content, description, is_favourite, last_used_at
        FROM commands
        WHERE group_id = ?
        ORDER BY id DESC
        "#,
    )
    .bind(group_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let tags = load_command_tags(pool.clone(), row.id).await?;
        out.push((row, tags));
    }
    Ok(out)
}

async fn load_command_tags(pool: Arc<SqlitePool>, command_id: i32) -> Result<Vec<String>, String> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM command_tags WHERE command_id = ? ORDER BY name",
    )
    .bind(command_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows.into_iter().map(|(n,)| n).collect())
}

pub async fn create_command(
    pool: Arc<SqlitePool>,
    group_id: i32,
    content: String,
    description: String,
    tags: Vec<String>,
) -> Result<i32, String> {
    let content = content.trim().to_string();
    if content.is_empty() {
        return Err("Command content is required".into());
    }

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let result = sqlx::query(
        r#"
        INSERT INTO commands (group_id, content, description)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(group_id)
    .bind(&content)
    .bind(&description)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let id = result.last_insert_rowid() as i32;

    for tag in tags {
        let tag = tag.trim().to_string();
        if tag.is_empty() {
            continue;
        }
        sqlx::query(
            "INSERT OR IGNORE INTO command_tags (command_id, name) VALUES (?, ?)",
        )
        .bind(id)
        .bind(&tag)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(id)
}

pub async fn update_command(
    pool: Arc<SqlitePool>,
    id: i32,
    content: String,
    description: String,
    tags: Vec<String>,
) -> Result<(), String> {
    let content = content.trim().to_string();
    if content.is_empty() {
        return Err("Command content is required".into());
    }

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    sqlx::query(
        r#"
        UPDATE commands
        SET content = ?, description = ?, updated_at = datetime('now')
        WHERE id = ?
        "#,
    )
    .bind(&content)
    .bind(&description)
    .bind(id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query("DELETE FROM command_tags WHERE command_id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    for tag in tags {
        let tag = tag.trim().to_string();
        if tag.is_empty() {
            continue;
        }
        sqlx::query(
            "INSERT OR IGNORE INTO command_tags (command_id, name) VALUES (?, ?)",
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

pub async fn delete_command(pool: Arc<SqlitePool>, id: i32) -> Result<(), String> {
    sqlx::query("DELETE FROM commands WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
