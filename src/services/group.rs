use std::sync::Arc;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, FromRow)]
pub struct GroupRow {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub item_count: i32,
}

// Special groups doesn't exists in the database and are created dynamically
pub const DEFAULT_GROUP_ID: i32 = 1;
pub const RECENT_GROUP_ID: i32 = -1;
pub const FAVOURITES_GROUP_ID: i32 = -2;

pub async fn load_groups(pool: Arc<SqlitePool>) -> Result<Vec<GroupRow>, String> {
    // Load regular groups
    let mut rows = sqlx::query_as::<_, GroupRow>(
        r#"
        SELECT
            g.id,
            g.name,
            g.description,
            (SELECT COUNT(*) FROM items s WHERE s.group_id = g.id) AS item_count
        FROM groups AS g
        ORDER BY g.name
        "#,
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    // Create special groups

    // Recent item count
    let recent_count: i32 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM items WHERE last_used_at IS NOT NULL",
    )
    .fetch_one(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    // Favourite item count
    let favourite_count: i32 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM items WHERE is_favourite = 1",
    )
    .fetch_one(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut out = Vec::with_capacity(rows.len() + 2);

    // Add special groups
    out.push(GroupRow {
        id: RECENT_GROUP_ID,
        name: "Recent".into(),
        description: "Echoes of your latest thoughts linger here - the spells you whispered to the machine, still warm, still within reach.".into(),
        item_count: recent_count,
    });

    out.push(GroupRow {
        id: FAVOURITES_GROUP_ID,
        name: "Favourites".into(),
        description: "The ones you chose to keep close - fragments of spells that earned your trust and found a home.".into(),
        item_count: favourite_count,
    });

    // Add regular groups
    out.append(&mut rows);

    Ok(out)
}

pub async fn create_group(
    pool: Arc<SqlitePool>,
    name: String,
    description: String,
) -> Result<i32, String> {
    let (name, description) = validate_fields(name, description)?;

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
    let (name, description) = validate_fields(name, description)?;

    sqlx::query(
        r#"
        UPDATE groups
        SET name = ?, description = ?
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
    if id == RECENT_GROUP_ID {
        return Err("Cannot delete 'Recent' group".into());
    }

    if id == FAVOURITES_GROUP_ID {
        return Err("Cannot delete 'Favourites' group".into());
    }

    if id == DEFAULT_GROUP_ID {
        return Err("Cannot delete 'Default' group".into());
    }

    sqlx::query("DELETE FROM groups WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn validate_fields(
    name: String,
    description: String,
) -> Result<(String, String), String> {
    let name = name.trim().to_string();
    let description = description.trim().to_string();

    if name.is_empty() {
        return Err("Group name is required".into());
    }

    if name.len() > 50 {
        return Err("Group name is too long (max 50 chars)".into());
    }

    if name == "Recent" || name == "Favourites" || name == "Default" {
        return Err(format!("Group name '{}' is reserved", name));
    }

    if description.len() > 1000 {
        return Err("Group description is too long".to_string());
    }

    Ok((name, description))
}
