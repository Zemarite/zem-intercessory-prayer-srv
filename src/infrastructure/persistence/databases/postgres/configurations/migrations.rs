use sqlx::PgPool;

/// Runs all pending database migrations.
/// This function should be called after creating the connection pool.
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
