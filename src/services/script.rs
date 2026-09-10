use std::sync::Arc;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, FromRow)]
pub struct ScriptRow {
    pub id: i32,
    pub group_id: i32,
    pub name: String,
    pub content: String,
    pub description: String,
    pub is_favourite: i32,
    pub last_used_at: Option<String>,
}

pub async fn load_scripts_for_group(
    pool: Arc<SqlitePool>,
    script_id: i32,
) -> Result<Vec<(ScriptRow, Vec<String>)>, String> {
    let rows = sqlx::query_as::<_, ScriptRow>(
        r#"
        SELECT id, group_id, name, content, description, is_favourite, last_used_at
        FROM scripts
        WHERE group_id = ?
        ORDER BY id DESC
        "#,
    )
    .bind(script_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let tags = load_script_tags(pool.clone(), row.id).await?;
        out.push((row, tags));
    }
    Ok(out)
}

async fn load_script_tags(pool: Arc<SqlitePool>, script_id: i32) -> Result<Vec<String>, String> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM script_tags WHERE script_id = ? ORDER BY name",
    )
    .bind(script_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows.into_iter().map(|(n,)| n).collect())
}

pub async fn create_script(
    pool: Arc<SqlitePool>,
    group_id: i32,
    name: String,
    content: String,
    description: String,
    tags: Vec<String>,
) -> Result<i32, String> {
    let name = name.trim().to_string();
    if name.is_empty() {
        return Err("Script name is required".into());
    }

    let content = content.trim().to_string();
    if content.is_empty() {
        return Err("Script content is required".into());
    }

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let result = sqlx::query(
        r#"
        INSERT INTO scripts (group_id, name, content, description)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(group_id)
    .bind(name)
    .bind(content)
    .bind(description)
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
            "INSERT OR IGNORE INTO script_tags (script_id, name) VALUES (?, ?)",
        )
        .bind(id)
        .bind(tag)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(id)
}

pub async fn update_script(
    pool: Arc<SqlitePool>,
    id: i32,
    name: String,
    content: String,
    description: String,
    tags: Vec<String>,
) -> Result<(), String> {
    let name = name.trim().to_string();
    if name.is_empty() {
        return Err("Script name is required".into());
    }

    let content = content.trim().to_string();
    if content.is_empty() {
        return Err("Script content is required".into());
    }

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    sqlx::query(
        r#"
        UPDATE scripts
        SET name = ?, content = ?, description = ?, updated_at = datetime('now')
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

    sqlx::query("DELETE FROM script_tags WHERE script_id = ?")
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
            "INSERT OR IGNORE INTO script_tags (script_id, name) VALUES (?, ?)",
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

pub async fn delete_script(pool: Arc<SqlitePool>, id: i32) -> Result<(), String> {
    sqlx::query("DELETE FROM scripts WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
