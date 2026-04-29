use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/// Configuration for PostgreSQL database connection.
#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub database_url: String,
    pub max_connections: u32,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            database_url: "postgres://localhost/zem_intercessory_prayer".to_string(),
            max_connections: 5,
        }
    }
}

impl PostgresConfig {
    /// Creates a new PostgreSQL configuration.
    pub fn new(database_url: String, max_connections: u32) -> Self {
        Self {
            database_url,
            max_connections,
        }
    }

    /// Creates a database connection pool.
    pub async fn create_pool(&self) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(self.max_connections)
            .connect(&self.database_url)
            .await
    }
}
