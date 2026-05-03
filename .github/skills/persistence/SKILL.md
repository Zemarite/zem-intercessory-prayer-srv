---
name: persistence
description: "**WORKFLOW SKILL** — Implement the data persistence layer using Rust, SQLx, and PostgreSQL. USE FOR: creating database connection pools, managing migrations, implementing repository patterns for domain entities, and ensuring robust error handling in the persistence layer. DO NOT USE FOR: domain logic, application services, or non-SQLx persistence. INVOKES: file system tools for code generation, read_file for domain traits."
---

# Infrastructure/Persistence Implementation

## Quick Start

- **When to Invoke**: Use this skill when tasked with implementing or updating the data persistence layer, including database connections, migrations, and repository implementations. Avoid for domain logic or application services.
- **Prerequisites**: Ensure the `domain/repositories/` traits are defined. Have database schema requirements ready for migration design.
- **Output**: Generates Rust files in the `persistence/` folder structure for database connections, migrations, repository, and errors.rs implementations.
- **Unit Testing**: Write unit tests for repository methods to ensure correct database interactions.
- **Validation**: After generation, run tests and check for SQLx compile errors to ensure queries are valid.

This skill provides a comprehensive workflow for implementing the data persistence layer using SQLx and PostgreSQL in pure Rust without any external ORM frameworks. It focuses on creating repository implementations that map between database models and domain entities, managing database connections, and handling migrations.

## Project Structure

```
persistence/
├── databases/
    └── postgres/
        └── configurations/
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

## Workflow

1. **Analyze Requirements**: Review domain repository traits to understand expected interfaces. Gather database schema requirements for migration design.
2. **Set Up Connection Pool**: Implement connection pooling using SQLx's `PgPool` in `connection.rs`.
3. **Design Migrations**: Create SQL migration files based on schema requirements and implement migration management in `migrations.rs`.
4. **Implement Repositories**: For each domain entity, implement the corresponding repository in `repositories/postgres/`, including DTOs, mapping, and repository logic.
5. **Handle Errors**: Define custom error types in `errors.rs` for robust error handling in the persistence layer.
6. **Organize Modules**: Update `mod.rs` files to ensure proper module exports and imports.
7. **Test and Validate**: Write unit tests for repository methods and run `cargo test` to validate functionality and catch SQLx compile errors.

## Interactive Checklist

Use this checklist to guide domain implementation. Complete each step systematically.

## Key Responsibilities

### 1. Connection Pool

- [ ] Implement connection pooling using SQLx's `PgPool` for efficient database connections.
- [ ] Configure database connection strings securely using environment variables or configuration files.

### 2. Migrations

- [ ] Create SQL migration files for database schema management.
- [ ] Implement migration management logic to apply migrations in order.

### 3. Repository Implementations

- [ ] Defined in the domain layer, implement repository traits for each entity in the `repositories/postgres/` folder.
- [ ] Implement repository traits defined in the domain layer for each entity.
- [ ] Create DTOs for database models and map them to domain entities.
- [ ] Use SQLx macros for compile-time checked queries in repository methods.
- [ ] Ensure proper error handling and propagation in repository methods.
- [ ] Define custom error types in `errors.rs` for robust error handling in the persistence layer.
- [ ] Write integration tests for repository operations to validate database interactions.

### 4. Module Exports

- [ ] Organize code with clear module exports for easy imports in application services and flatening the structure for ease of use
- [ ] Update `mod.rs` files to ensure all necessary modules are exported correctly.

### 5. Testing and Validation

- [ ] Write unit tests for repository constructors, methods, and invariants to ensure correct database interactions.
- [ ] Run `cargo test` to validate functionality and catch SQLx compile errors.
- [ ] Ensure tests cover edge cases, such as handling database connection failures or invalid queries.

## Implementation Guidelines

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
