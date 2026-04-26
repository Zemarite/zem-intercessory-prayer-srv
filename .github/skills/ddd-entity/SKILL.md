---
name: ddd-entity
description: "**WORKFLOW SKILL** — Generate complete Domain-Driven Design entity files in pure Rust without frameworks. USE FOR: creating entities, value objects, domain events, and repositories in the DDD domain layer; populating the domain folder structure with pure Rust implementations; ensuring entity purity and immutability."
---

# Rust DDD Entity Pure Implementation

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

1. **Define Domain Concepts**: Identify entities, value objects, and events from business requirements
2. **Create Value Objects**: Implement immutable types with validation
3. **Implement Entities**: Define aggregate roots with business methods
4. **Define Enums**: Add domain-specific enumerations
5. **Add Domain Events**: Create events for state changes
6. **Create Repository Interfaces**: Define traits for data access
7. **Implement Error Types**: Add domain-specific error handling
8. **Update Module Structure**: Ensure proper module exports
9. **Write Unit Tests**: Focus on domain logic and invariants

## Interactive Checklist

Use this checklist to guide domain implementation. Complete each step systematically.

### 1. Domain Analysis

- [ ] Business requirements documented
- [ ] Bounded context boundaries defined
- [ ] Key domain concepts identified (entities, values, events)

### 2. Value Objects

- [ ] Value object structs created in `value_objects/`
- [ ] Validation logic implemented in `new()` constructors
- [ ] `PartialEq`, `Eq`, `Hash` derived where appropriate
- [ ] Domain methods added for business operations
- [ ] Unit tests written for validation and methods

### 3. Entities

- [ ] Entity structs defined in `entities/` with ID fields
- Entities have identity and lifecycle.
- Aggregate roots should protect invariants and control access to internal state.
- Business rules should live here when they naturally belong to the entity.
- [ ] Entities Structure:
    - [ ] Structs: deriving `Debug`, `Clone`, `PartialEq`, and `Eq` where appropriate. Implement domain behavior as methods on the entity, ensuring that invariants are maintained.
    - [ ] Impl: blocks contain the methods implementing business rules and behavior.
        - [ ] Factory: methods for creation (e.g., `new()`) that enforce valid state.
        - [ ] Business behavior / methods: that enforce invariants and guard state transitions (e.g., `rename()`, `update_status()`, `change_email()`).
        - [ ] Validation: logic to ensure entities cannot enter invalid states. This can be done through constructors or methods that return `Result<(), DomainError>`.
        - [ ] Getters: Provide read-only access to fields. Returning references avoids cloning (e.g., &str for String).
    - [ ] Impl `PartialEq` and `Eq`: based on identity fields (e.g., `id`) rather than all fields, to reflect the concept of entity identity in DDD.

### 4. Domain Enums

- [ ] Enumeration types defined in `enums/`
- [ ] Variant names follow domain terminology
- [ ] Methods added for enum-specific logic
- [ ] impl `as_str()` human-readable string representation
- [ ] impl `from_str()` create a ContactMethod from a string
- [ ] `Display` and `FromStr` implemented where useful

### 5. Domain Events

- [ ] Event structs created in `events/` with timestamp
- [ ] Event data includes relevant entity state
- [ ] Serialization traits implemented if needed
- [ ] Event naming follows past tense convention
- [ ] Unit tests for event creation

### 6. Repository Interfaces

- [ ] Repository traits defined in `repositories/`
- [ ] Create and update operations separated in method signatures
- [ ] Method signatures follow CQRS principles (commands/queries)
- [ ] Error types specified for each operation
- [ ] Generic constraints added for type safety
- [ ] Associated types used for complex return values

### 7. Error Handling

- [ ] Domain error enum created in `errors.rs`
- [ ] Error variants cover all domain failure modes
- [ ] Error conversion implementations added

- **Clarity and Precision:** Using enum variants with descriptive names (and favoring struct variants over tuple variants) ensures that the code remains clear, rather than relying on ambiguous text or tuple structures that hide the meaning of the data.
- **Consistency:** Using an enum allows for a structured approach that can be applied consistently across the project as it matures.
- **Programmatic Utility:** By avoiding reliance on display text for error meaning, the code remains more useful for the machine; for instance, the error can be serialized to JSON for logging or client-side rendering in web or device applications.

### 8. Module Organization

- [ ] All modules declared in `mod.rs`
- [ ] Public API clearly defined
- [ ] Re-exports added for convenience
- [ ] Documentation comments added to public items

### 9. Testing & Validation

- [ ] Unit tests written for all domain logic in entities, value objects, events and enums
- [ ] Edge cases and invariants tested
- [ ] Test coverage reviewed for critical paths
- [ ] Property-based testing considered for complex logic
- [ ] Domain rules verified through tests
- [ ] Code review completed for architectural compliance

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

pub type result<T> = core::result::Result<T, DomainError>;

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

### Overall Guidance

- **Purity:** The domain layer should not depend on web frameworks, database libraries, or transport models.
- **CQRS Alignment:** Commands and queries live outside the domain, but both rely on the same domain rules and models.
- **Testing:** Prefer unit tests for entities and value objects since domain logic should be deterministic and easy to test.
- **Evolution:** Start simple, then introduce additional domain concepts such as domain services or policies only when the model needs them.
- **Performance:** Keep the domain focused on correctness and clarity first; optimization belongs elsewhere unless a business rule truly requires it.

### Important Boundaries

The domain layer should **not** contain:

- HTTP request/response models
- application handlers
- database-specific code
- framework middleware
- serialization concerns unless truly required by the domain

The domain layer **should** contain:

- business rules
- invariants
- rich models
- domain terminology
- contracts that express what the business needs

## Best Practices

- Keep domain layer free of external dependencies
- Use newtypes for primitive type safety
- Prefer associated functions over constructors for complex creation
- Implement `Display` for user-friendly representations
- Use `PhantomData` for marker traits if needed
- Separate create and update operations in repositories to clarify intent and enable different validation/authorization logic
- Document business rules in code comments
- Test domain logic thoroughly with edge cases
