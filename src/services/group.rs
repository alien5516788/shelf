use std::sync::Arc;
use sqlx::{FromRow, SqlitePool};


#[derive(Debug, Clone, FromRow)]
pub struct GroupRow {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub item_count: i64,
}

pub async fn load_groups(pool: Arc<SqlitePool>) -> Result<Vec<GroupRow>, String> {
    sqlx::query_as::<_, GroupRow>(
        r#"
        SELECT
            g.id,
            g.name,
            g.description,
            COUNT(i.id) AS item_count
        FROM group g
        LEFT JOIN items i ON i.group_id = g.id
        GROUP BY g.id
        ORDER BY
            CASE g.id
                WHEN 'recent'   THEN 0
                WHEN 'favorite' THEN 1
                ELSE 2
            END,
            g.name
        "#,
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())
}
