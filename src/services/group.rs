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
