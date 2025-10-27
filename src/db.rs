use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;
use std::time::Duration;

pub async fn init_pool(database_url: &str) -> anyhow::Result<SqlitePool> {
    tracing::info!("Connecting to database: {}", database_url);

    // Ensure the database directory exists
    if let Some(db_path) = extract_db_path(database_url) {
        if let Some(parent) = Path::new(&db_path).parent() {
            // Only create if parent is not current directory or empty
            if parent != Path::new("") && parent != Path::new(".") && !parent.exists() {
                std::fs::create_dir_all(parent)?;
                tracing::info!("Created database directory: {}", parent.display());
            }
        }
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await?;

    tracing::info!("Database connection pool created");

    run_migrations(&pool).await?;

    Ok(pool)
}

fn extract_db_path(database_url: &str) -> Option<String> {
    // Extract path from sqlite:// URLs
    if let Some(path) = database_url.strip_prefix("sqlite://") {
        // Remove query parameters if present
        let path = path.split('?').next().unwrap_or(path);
        Some(path.to_string())
    } else if let Some(path) = database_url.strip_prefix("sqlite:") {
        let path = path.split('?').next().unwrap_or(path);
        Some(path.to_string())
    } else {
        None
    }
}

async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    tracing::info!("Running database migrations...");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS secrets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT NOT NULL UNIQUE,
            ciphertext BLOB NOT NULL,
            maxviews INTEGER,
            exp INTEGER,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_secrets_uuid ON secrets(uuid)
        "#,
    )
    .execute(pool)
    .await?;

    tracing::info!("Database migrations completed");

    Ok(())
}
