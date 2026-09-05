use std::path::PathBuf;
use std::sync::Arc;

use sqlx::Error;
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

    // Seed default groups (idempotent)
    println!("Shelf: Seeding default groups");
    seed_default_groups(&pool)
        .await
        .map_err(|e| format!("Seeding failed: {e}"))?;

    Ok(Arc::new(pool))
}

fn database_path() -> Result<PathBuf, String> {
    let data_dir = ensure_data_dir()?;
    Ok(data_dir.join(DB_NAME))
}

async fn seed_default_groups(pool: &SqlitePool) -> Result<(), Error> {
    // Recent
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO groups (id, name, description)
        VALUES ('recent', 'Recent', 'Recently used commands and scripts')
        "#,
    )
    .execute(pool)
    .await?;

    // Favorite
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO groups (id, name, description)
        VALUES ('favorite', 'Favorite', 'Your favourite commands and scripts')
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
