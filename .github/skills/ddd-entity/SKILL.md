---
name: ddd-entity
description: "**WORKFLOW SKILL** — Generate complete Domain-Driven Design entity files in pure Rust without frameworks. USE FOR: creating entities, value objects, domain events, and repositories in the DDD domain layer; populating the domain folder structure with pure Rust implementations; ensuring entity purity and immutability."
---

# Rust DDD Entity Pure Implementation

## Quick Start

- **When to Invoke**: Use this skill when tasked with creating or updating DDD entities, value objects, events, or repositories in the `domain/` layer. Avoid for infrastructure (e.g., persistence) or application logic.
- **Prerequisites**: Ensure the `domain/` folder structure exists (see Project Structure). Have business requirements ready for domain analysis.
- **Output**: Generates pure Rust files in `domain/`, updates `mod.rs`, and includes unit tests.

This skill provides a comprehensive workflow for implementing Domain-Driven Design (DDD) entities and related domain objects in pure Rust without any external frameworks. It focuses on creating pure, immutable domain models that encapsulate business logic.

## Principles

- **Entity Purity**: Entities contain only business logic, no infrastructure concerns
- **Immutability**: Prefer immutable structs where possible, use methods for state changes
- **Strong Typing**: Leverage Rust's type system for domain invariants
- **No Frameworks**: Pure Rust code, no dependencies on web frameworks, ORMs, or serialization libraries in the domain layer

## Project Structure

```
domain/
├── entities/          # Aggregate roots and entities
├── value_objects/     # Immutable value types
├── enums/            # Domain enumerations
├── events/           # Domain events
├── repositories/     # Repository traits/interfaces
├── errors.rs         # Domain-specific errors
└── mod.rs            # Module declarations
```

## Workflow

Follow these steps to generate DDD entities. Use the Interactive Checklist for detailed tasks.

1. **Analyze Requirements**: Gather business rules, entities, and invariants from user input or docs. Identify aggregates and value objects.
2. **Generate Value Objects**: Create immutable types with validation (see Checklist Step 2).
3. **Generate Entities**: Implement aggregate roots with methods (see Checklist Step 3).
4. **Add Enums and Events**: Define domain enums and events (see Checklist Steps 4-5).
5. **Define Repositories**: Create trait interfaces (see Checklist Step 6).
6. **Handle Errors**: Implement domain errors (see Checklist Step 7).
7. **Organize Modules**: Update `mod.rs` and exports (see Checklist Step 8).
8. **Test and Validate**: Write unit tests and run `cargo test` (see Checklist Step 9).

## Interactive Checklist

Use this checklist to guide domain implementation. Complete each step systematically.

## Key Concepts

### Entities

Entities have identity and lifecycle. Aggregate roots should protect invariants and control access to internal state. Business rules should live here when they naturally belong to the entity.

### 1. Domain Analysis

- [ ] Document business requirements
- [ ] Define bounded context boundaries
- [ ] Identify key domain concepts (entities, values, events)

### 2. Value Objects

- [ ] Create value object structs in `value_objects/`
- [ ] Implement validation logic in `new()` constructors
- [ ] Derive `PartialEq`, `Eq`, `Hash` where appropriate
- [ ] Add domain methods for business operations
- [ ] Write unit tests for validation and methods
    - **ID types** (e.g., `ProgramId`): test `generate()` produces a valid UUID, `parse()` accepts valid UUIDs and rejects invalid strings, `value()` round-trips correctly, and `Display` formats as expected
    - **Other value objects**: test all constructor validation rules, edge cases (empty strings, max length boundaries), and any domain-specific methods

### 3. Entities

- [ ] Define entity structs in `entities/` with ID fields (e.g., `pub struct EntityName { id: EntityId, ... }`).
- [ ] Derive `Debug`, `Clone`, `PartialEq`, and `Eq` where appropriate.
- [ ] Implement factory methods (e.g., `new()`) that enforce valid initial state.
- [ ] Add business methods (e.g., `rename()`, `update_status()`) that maintain invariants and return `Result<(), DomainError>`.
- [ ] Implement getters for read-only field access (e.g., return `&str` for strings).
- [ ] Implement `PartialEq` and `Eq` based on identity (e.g., `id` field) only.
- [ ] Write unit tests for creation, methods, and invariants.

### 4. Domain Enums

- [ ] Define enumeration types in `enums/`
- [ ] Use variant names that follow domain terminology
- [ ] Add methods for enum-specific logic
- [ ] Implement `as_str()` for human-readable string representation
- [ ] Implement `from_str()` to create the enum from a string
- [ ] Implement `Display` and `FromStr` where useful

### 5. Domain Events

- [ ] Create event structs in `events/` with timestamp
- [ ] Include event data with relevant entity state
- [ ] Implement serialization traits if needed
- [ ] Follow past tense convention for event naming
- [ ] Write unit tests for event creation

### 6. Repository Interfaces

- [ ] Define repository traits in `repositories/`
- [ ] Separate create and update operations in method signatures
- [ ] Ensure method signatures follow CQRS principles (commands/queries)
- [ ] Specify error types for each operation
- [ ] Add generic constraints for type safety
- [ ] Use associated types for complex return values

### 7. Error Handling

- [ ] Create domain error enum in `errors.rs`
- [ ] Cover all domain failure modes with error variants
- [ ] Add error conversion implementations

### 8. Module Organization

- [ ] Declare all modules in `mod.rs`
- [ ] Define public API clearly
- [ ] Add re-exports for convenience
- [ ] Add documentation comments to public items

### 9. Testing & Validation

- [ ] Write unit tests for all domain logic in entities, value objects, events and enums
- [ ] Test edge cases and invariants
- [ ] Review test coverage for critical paths
- [ ] Consider property-based testing for complex logic
- [ ] Verify domain rules through tests
- [ ] Complete code review for architectural compliance

## Code Templates

### Repository Trait Template

```rust
use async_trait::async_trait;
use crate::domain::{entities::User, value_objects::UserId};

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: User) -> Result<(), RepositoryError>;
    async fn update(&self, user: &User) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, RepositoryError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, RepositoryError>;
    async fn list_all(&self) -> Result<Vec<User>, RepositoryError>;
    async fn delete(&self, id: &UserId) -> Result<(), RepositoryError>;
}
```

### Error Template

```rust
use derive_more::{Display, From};

pub type Result<T> = core::result::Result<T, DomainError>;

#[derive(Debug, Display, From)]
#[display("{self:?}")]
pub enum DomainError {
    InvalidEmail(String),

    #[from(String, &String, &str)]
    Custom(String),

    // -- Externals
    #[from]
    Io(std::io::Error), // as example

}

impl DomainError {
    pub fn custom(err: impl std::error::Error) -> Self {
        Self::Custom(err.to_string())
    }

    pub fn custom(val: impl Into<String>) -> Self {
        Self::Custom(val.into())
    }
}

impl std::error::Error for DomainError {}
```

## Best Practices

- Keep domain layer free of external dependencies
- Use newtypes for primitive type safety
- Prefer associated functions over constructors for complex creation
- Implement `Display` for user-friendly representations
- Use `PhantomData` for marker traits if needed
- Separate create and update operations in repositories to clarify intent and enable different validation/authorization logic
- Document business rules in code comments
- Test domain logic thoroughly with edge cases
- Ensure the domain layer does not depend on web frameworks, database libraries, or transport models (purity)
- Commands and queries live outside the domain, but both rely on the same domain rules and models (CQRS alignment)
- Prefer unit tests for entities and value objects since domain logic should be deterministic and easy to test
- Start simple, then introduce additional domain concepts such as domain services or policies only when the model needs them (evolution)
- Keep the domain focused on correctness and clarity first; optimization belongs elsewhere unless a business rule truly requires it (performance)
- The domain layer should not contain HTTP request/response models, application handlers, database-specific code, framework middleware, or serialization concerns unless truly required by the domain
- The domain layer should contain business rules, invariants, rich models, domain terminology, and contracts that express what the business needs

### Testing Guidance

- For enums containing non-PartialEq fields (e.g., std::io::Error), prefer `assert!(matches!(...))` over `assert_eq!` in tests unless implementing manual PartialEq.
- Write unit tests for all constructors, methods, and invariants.
- Use property-based testing for complex validation logic.

### Integration Notes

- After generation, run `cargo check` and update `src/lib.rs` or `main.rs` if needed.
- Ensure no external imports in domain layer.

## Troubleshooting

- If tests fail, check invariants in entity methods.
- For compilation errors, verify no external imports in the domain layer.
