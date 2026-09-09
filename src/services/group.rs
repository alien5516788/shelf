use std::sync::Arc;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, FromRow)]
pub struct GroupRow {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub item_count: i32,
}

pub async fn load_groups(pool: Arc<SqlitePool>) -> Result<Vec<GroupRow>, String> {
    sqlx::query_as::<_, GroupRow>(
        r#"
        SELECT
            g.id,
            g.name,
            g.description,
            (
                SELECT COUNT(*) FROM commands c WHERE c.group_id = g.id
            ) + (
                SELECT COUNT(*) FROM scripts s WHERE s.group_id = g.id
            ) AS item_count
        FROM groups AS g
        ORDER BY g.name
        "#,
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())
}

pub async fn create_group(
    pool: Arc<SqlitePool>,
    name: String,
    description: String,
) -> Result<i32, String> {
    let name = name.trim().to_string();
    if name.is_empty() {
        return Err("Group name is required".into());
    }

    let result = sqlx::query(
        r#"
        INSERT INTO groups (name, description)
        VALUES (?, ?)
        "#,
    )
    .bind(&name)
    .bind(&description)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(result.last_insert_rowid() as i32)
}

pub async fn update_group(
    pool: Arc<SqlitePool>,
    id: i32,
    name: String,
    description: String,
) -> Result<(), String> {
    let name = name.trim().to_string();
    if name.is_empty() {
        return Err("Group name is required".into());
    }

    sqlx::query(
        r#"
        UPDATE groups
        SET name = ?, description = ?, updated_at = datetime('now')
        WHERE id = ?
        "#,
    )
    .bind(&name)
    .bind(&description)
    .bind(id)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn delete_group(pool: Arc<SqlitePool>, id: i32) -> Result<(), String> {
    sqlx::query("DELETE FROM groups WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
