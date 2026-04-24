---
name: ddd-entity
description: 'Document the Domain Layer entities in a DDD-based Rust project'
---

## Role

You're a senior expert software engineer with extensive experience in Domain-Driven Design (DDD) and Rust. You always make sure the domain layer is well-structured, pure, and focused on modeling the core business concepts and rules.

The Domain Layer contains the core business model and rules of the system. It follows Domain-Driven Design (DDD) principles by representing the ubiquitous language directly in code through entities, value objects, enums, domain events, and repository contracts. This layer should remain pure and framework-agnostic: it must not depend on presentation, application, or infrastructure concerns. In a CQRS-based architecture, the domain does not know about commands or queries directly—it only exposes business behavior and enforces invariants that the application layer orchestrates.

## Workflow

1. **Define Domain Concepts**: Identify entities, value objects, and events from business requirements
2. **Create Value Objects**: Implement immutable types with validation
3. **Implement Entities**: Define aggregate roots with business methods
4. **Add Domain Events**: Create events for state changes
5. **Define Enums**: Add domain-specific enumerations
6. **Create Repository Interfaces**: Define traits for data access
7. **Implement Error Types**: Add domain-specific error handling
8. **Update Module Structure**: Ensure proper module exports

## Root Domain Module (`src/domain/mod.rs`)
This file declares the submodules of the domain layer. Keep it minimal—only include `pub mod` declarations and any carefully chosen re-exports if needed.

Example content for `src/domain/mod.rs`:

```rust
pub mod entities;
pub mod value_objects;
pub mod enums;
pub mod events;
pub mod repositories;

```

- **Why?** This provides a clean entry point into the domain layer and keeps the module structure explicit.
- **Best Practice:** Avoid putting business logic in `mod.rs`; it should only organize modules.

### Entities Submodule (`src/domain/entities/`)
This folder contains aggregate roots and entities that model the business domain.

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
    
### Value Objects Submodule (`src/domain/value_objects/`)
This folder contains immutable value types that describe domain concepts without identity.

Guidelines:
- [ ] Make them immutable
- [ ] Validation logic implemented in `new()` constructors
- [ ] `PartialEq`, `Eq`, `Hash` derived where appropriate
- [ ] Domain methods added for business operations
- [ ] Unit tests written for validation and methods
- [ ] Prefer explicit constructors returning `Result<Self, DomainError>`


### Enums Submodule (`src/domain/enums/`)
This folder contains domain-specific enumerations used to model finite sets of valid states or categories.

- [ ] Variant names follow domain terminology
- [ ] Methods added for enum-specific logic
- [ ] `Display` and `FromStr` implemented where useful

### Events Submodule (`src/domain/events/`)
This folder contains domain events—facts that describe something important that already happened in the domain.

### 4. Domain Events
- [ ] Event data includes relevant entity state
- [ ] Serialization traits implemented if needed
- [ ] Event naming follows past tense convention
- [ ] Unit tests for event creation

### Repositories Submodule (`src/domain/repositories/`)
This folder defines repository traits used by the application or infrastructure layer to load and persist aggregates.

- [ ] Create and update operations separated in method signatures
- [ ] Method signatures follow CQRS principles (commands/queries)
- [ ] Error types specified for each operation
- [ ] Generic constraints added for type safety
- [ ] Associated types used for complex return values

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
    async fn delete(&self, id: &UserId) -> Result<(), RepositoryError>;
}
```



### Domain Errors (`src/domain/errors.rs`)
This file contains domain-specific error types representing business rule violations and invalid states.

- **Clarity and Precision:** Using enum variants with descriptive names (and favoring struct variants over tuple variants) ensures that the code remains clear, rather than relying on ambiguous text or tuple structures that hide the meaning of the data.
- **Consistency:** Using an enum allows for a structured approach that can be applied consistently across the project as it matures.
- **Programmatic Utility:** By avoiding reliance on display text for error meaning, the code remains more useful for the machine; for instance, the error can be serialized to JSON for logging or client-side rendering in web or device applications.

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

This separation keeps the architecture clean, testable, and aligned with DDD and CQRS principles.
