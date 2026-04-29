---
name: persistence
description: "**WORKFLOW SKILL** — Implement the data persistence layer using Rust, SQLx, and PostgreSQL. USE FOR: creating database connection pools, managing migrations, implementing repository patterns for domain entities, and ensuring robust error handling in the persistence layer. DO NOT USE FOR: domain logic, application services, or non-SQLx persistence. INVOKES: file system tools for code generation, read_file for domain traits."
---

# Infrastructure/Persistence Implementation

## Quick Start

- **When to Invoke**: Use this skill when tasked with implementing or updating the data persistence layer, including database connections, migrations, and repository implementations. Avoid for domain logic or application services.
- **Prerequisites**: Ensure the `domain/repositories/` traits are defined. Have database schema requirements ready for migration design.
- **Output**: Generates Rust files for database connections, migrations, repository and errors.rs implementations in the `persistence/` folder structure.
- **Validation**: After generation, run tests and check for SQLx compile errors to ensure queries are valid.

## Project Structure

```
persistence/
├── databases/
    └── postgres/
        └── configurations/                 # Connection pools and initialization
        │   ├── connection.rs               # Connection pools and initialization
        │   ├── migrations.rs               # Database migrations management
        │   └── mod.rs                      # Module exports
        └── repositories/
        │   ├── mod.rs                      # Module exports
        │   └── {entity}_repository/        # Repository implementations
        │       ├── dtos.rs                 # Data Transfer Objects for database models
        │       ├── mod.rs                  # Module exports
        │       ├── mapping.rs              # Mapping between database models and domain entities
        │       └── {entity}_repository.rs  # Repository implementations
        ├── mod.rs                          # Module exports
        └── errors.rs                       # Custom error types for persistence layer

```

Adapt paths based on codebase conventions; use tools to verify existing structure.

## Key Responsibilities

### 1. Database Layer (`databases/postgres/`)

- **Connection Pooling**: Use SQLx's `PgPool` for managing PostgreSQL connections
- **Migrations**: Implement versioned SQL migrations for schema management
- **Query Building**: Leverage SQLx's compile-time query checking
- **DTOs and Mapping**: Create DTOs for database models and map them to domain entities
- **Module Exports**: Organize code with clear module exports for easy imports in application services and flatening the structure for ease of use
- **Error Handling**: Proper error propagation and handling
    - /errors-rs infrastructure database postgres

### 2. Repository Pattern (`repositories/postgres/`)

- Implement repository traits defined in the domain layer
- Execute SQLx queries against the database
- Map database models to domain entities
- Handle transaction management where needed

## Implementation Guidelines

### Technologies

- **SQLx**: Async SQL toolkit with compile-time checked queries
- **PostgreSQL**: Primary database
- **tokio**: Async runtime (paired with SQLx)
- **sqlx-migrations**: Database version control

### Dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
sqlx = { version = "0.8.6", features = ["postgres", "runtime-tokio", "macros"] }
tokio = { version = "1.52.1", features = ["full"] }
async-trait = "0.1"
```

### Connection Pool Example

```rust
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool(database_url: &str) -> Result<sqlx::PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}
```

### Repository Implementation Pattern

```rust
use sqlx::PgPool;
use crate::domain::repositories::YourRepository;

pub struct PostgresYourRepository {
    pool: PgPool,
}

impl PostgresYourRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl YourRepository for PostgresYourRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<YourEntity>, PersistenceError> {
        sqlx::query_as!(YourEntity, "SELECT * FROM your_table WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(PersistenceError::from)
    }
    // Implement other trait methods using sqlx queries
}
```

### Migration Example

Create a migration file in `migrations/` (e.g., `001_create_users.sql`):

```sql
-- Create users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## Checklist

- [ ] Review domain repository traits to understand expected interfaces
- [ ] Set up PostgreSQL connection pool initialization
- [ ] Create SQL migration files for required schema
- [ ] Implement repository pattern for each domain entity
- [ ] Add proper error handling and logging
- [ ] Write integration tests for repository operations
- [ ] Configure database connection strings in environment

## Edge Cases and Common Pitfalls

- **Edge Case: Handling non-PostgreSQL databases** — Adapt SQLx to other drivers (e.g., MySQL) if needed, but default to PostgreSQL for best compile-time safety.
- **Pitfall: Skipping compile-time checks** — Always use SQLx macros (e.g., `query_as!`) to catch query errors at compile time; avoid raw strings.
- **Avoid: Mixing persistence logic with domain rules** — Keep repositories focused on data access; delegate business logic to domain services.
- **Pitfall: Ignoring transaction management** — Use SQLx transactions for multi-step operations to ensure atomicity.

## Resources

- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [SQLx Query Macros](https://github.com/launchbadge/sqlx/blob/main/sqlx-macros/README.md)
- [PostgreSQL Type Support in SQLx](https://docs.rs/sqlx/latest/sqlx/)
- [Async Traits with async_trait](https://docs.rs/async-trait/latest/async_trait/)

## Version/Updates

Last reviewed April 27, 2026. Update if SQLx versions change or new best practices emerge.
