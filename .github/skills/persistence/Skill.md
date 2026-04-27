# Infrastructure/Persistence Skill

## Overview
Implement data persistence layer using Rust, SQLx, and PostgreSQL for the Zem Intercessory Prayer Service.

## Project Structure
```
persistence/
├── databases/
│   └── postgres/
│       ├── connection.rs          # Connection pools and initialization
│       ├── migrations.rs          # Database migrations management
│       └── mod.rs                 # Module exports
└── repositories/
    └── postgres/
        ├── mod.rs                 # Module exports
        └── {entity}_repository.rs # Repository implementations
```

## Key Responsibilities

### 1. Database Layer (`databases/postgres/`)
- **Connection Pooling**: Use SQLx's `PgPool` for managing PostgreSQL connections
- **Migrations**: Implement versioned SQL migrations for schema management
- **Query Building**: Leverage SQLx's compile-time query checking
- **Error Handling**: Proper error propagation and handling

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

### Dependencies (Cargo.toml)
```toml
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls", "migrate"] }
tokio = { version = "1", features = ["full"] }
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
    // Implement trait methods using sqlx queries
}
```

## Checklist
- [ ] Review domain repository traits to understand expected interfaces
- [ ] Set up PostgreSQL connection pool initialization
- [ ] Create SQL migration files for required schema
- [ ] Implement repository pattern for each domain entity
- [ ] Add proper error handling and logging
- [ ] Write integration tests for repository operations
- [ ] Configure database connection strings in environment

## Resources
- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [SQLx Query Macros](https://github.com/launchbadge/sqlx/blob/main/sqlx-macros/README.md)
- [PostgreSQL Type Support in SQLx](https://docs.rs/sqlx/latest/sqlx/)
- [Async Traits with async_trait](https://docs.rs/async-trait/latest/async_trait/)
