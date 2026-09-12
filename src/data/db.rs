use std::path::PathBuf;
use std::sync::Arc;

use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

use crate::data::ensure_data_dir;


const DB_NAME: &str = "shelf.db";

// Looks for migrations in the "./migrations" directory at compile time
static MIGRATOR: Migrator = sqlx::migrate!();


// Creates the pool, runs migrations and seeds the default groups
// Returns an Arc so it can be shared cheaply
pub async fn init_db() -> Result<Arc<SqlitePool>, String> {
    let db_path = database_path().map_err(|e| e.to_string())?;

    // Make sure the file exists
    if !db_path.exists() {
        std::fs::File::create(&db_path)
            .map_err(|e| format!("Failed to create database file: {e}"))?;
    }

    // Sqlite url
    let url = format!("sqlite:{}", db_path.display());
    println!("Shelf: Database url: {}", url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .map_err(|e| format!("Failed to connect to database: {e}"))?;

    // Run migrations
    println!("Shelf: Running migrations");
    MIGRATOR
        .run(&pool)
        .await
        .map_err(|e| format!("Migration failed: {e}"))?;

    // Create default group
    println!("Shelf: Creating default group");
    create_default_group(&pool)
        .await
        .map_err(|e| format!("Failed to create default group: {e}"))?;

    Ok(Arc::new(pool))
}

fn database_path() -> Result<PathBuf, String> {
    let data_dir = ensure_data_dir()?;
    Ok(data_dir.join(DB_NAME))
}

async fn create_default_group(pool: &SqlitePool) -> Result<(), String> {
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO groups (id, name, description)
        VALUES (1, 'Default', "This is where any item that doesn't fit to any other group belongs to.")
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| format!("{}", e))?;

    Ok(())
}
