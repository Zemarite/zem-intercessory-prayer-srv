# Domain Layer

The Domain Layer contains the core business model and rules of the system. It follows Domain-Driven Design (DDD) principles by representing the ubiquitous language directly in code through entities, value objects, enums, domain events, and repository contracts. This layer should remain pure and framework-agnostic: it must not depend on presentation, application, or infrastructure concerns. In a CQRS-based architecture, the domain does not know about commands or queries directly—it only exposes business behavior and enforces invariants that the application layer orchestrates.

## Root Domain Module (`src/domain/mod.rs`)
This file declares the submodules of the domain layer. Keep it minimal—only include `pub mod` declarations and any carefully chosen re-exports if needed.

Example content for `src/domain/mod.rs`:

```rust
pub mod entities;
pub mod value_objects;
pub mod enums;
pub mod events;
pub mod repositories;
pub mod errors;
```

- **Why?** This provides a clean entry point into the domain layer and keeps the module structure explicit.
- **Best Practice:** Avoid putting business logic in `mod.rs`; it should only organize modules.

### Entities Submodule (`src/domain/entities/`)
This folder contains aggregate roots and entities that model the business domain.

- Entities have identity and lifecycle.
- Aggregate roots should protect invariants and control access to internal state.
- Business rules should live here when they naturally belong to the entity.
- Entities Structure:
    - Structs: deriving `Debug`, `Clone`, `PartialEq`, and `Eq` where appropriate. Implement domain behavior as methods on the entity, ensuring that invariants are maintained.
    - Impl: blocks contain the methods implementing business rules and behavior.
      - Factory: methods for creation (e.g., `new()`) that enforce valid state.
      - Business behavior / methods: that enforce invariants and guard state transitions (e.g., `rename()`, `update_status()`, `change_email()`).
      - Validation: logic to ensure entities cannot enter invalid states. This can be done through constructors or methods that return `Result<(), DomainError>`.
      - Getters: Provide read-only access to fields. Returning references avoids cloning (e.g., &str for String). 
    - Impl `PartialEq` and `Eq`: based on identity fields (e.g., `id`) rather than all fields, to reflect the concept of entity identity in DDD.
    
Example responsibilities:
- Enforcing valid state transitions
- Guarding invariants
- Recording domain events when important changes occur

- **Principles Alignment:** DDD places core business behavior inside entities, not in handlers or controllers.
- **Gotcha:** Avoid anemic models. If entities only hold data and all rules live elsewhere, the domain becomes weak and harder to maintain.

### Value Objects Submodule (`src/domain/value_objects/`)
This folder contains immutable value types that describe domain concepts without identity.

Examples:
- `Email`, `BillingInfo`, `ProductId`, `UserId`, `OrderId`, `PhoneNumber`, `Currency`, `Percentage`, `DateRange`, `Address`, `PasswordHash`, `Slug`, `Url`, `Uuid`, `VersionNumber`, `Coordinates`, `Duration`, `Timestamp`, `MoneyAmount`

Guidelines:
- Make them immutable
- Validate them at creation time
- Derive traits like `Debug`, `Clone`, `PartialEq`, and `Eq` where appropriate
- Prefer explicit constructors returning `Result<Self, DomainError>`

- **Why?** Value objects make invalid states harder to represent and keep business concepts expressive.
- **Best Practice:** If a field has rules, it is often better modeled as a value object than a raw `String` or `i32`.

### Enums Submodule (`src/domain/enums/`)
This folder contains domain-specific enumerations used to model finite sets of valid states or categories.

Examples:
- `UserRole`, `OrderState`, `PaymentStatus`, `SubscriptionPlan`, `NotificationType`, `AccountType`, `ProductCategory`, `ShippingMethod`, `CustomerSegment`, `AccessLevel`, `PriorityLevel`, `TaskStatus`, `EventType`, `LogLevel`, `DeviceType`, `OperatingSystem`, `BrowserType`, `LanguagePreference`, `CurrencyCode`, `TimeZone`
- `PaymentMethod`

 
- **Why?** Enums make state explicit and safer than magic strings or numbers.
- **Best Practice:** Use meaningful variant names and attach behavior with `impl` blocks when useful.

### Events Submodule (`src/domain/events/`)
This folder contains domain events—facts that describe something important that already happened in the domain.

Examples:
- `UserRegistered`, `OrderPlaced`, `EmailChanged`, `PasswordUpdated`, `UserDeactivated`, `OrderShipped`, `PaymentFailed`, `InventoryReserved`, `CustomerNotified`, `SubscriptionRenewed`, `ProfileUpdated`, `AccountLocked`, `PasswordResetRequested`, `TwoFactorEnabled`, `TwoFactorDisabled`, `AddressAdded`, `AddressRemoved`, `ProductAddedToCart`, `ProductRemovedFromCart`, `CheckoutInitiated`, `CheckoutCompleted`, `RefundIssued`, `ReviewSubmitted`, `CommentPosted`, `MessageSent`, `NotificationSent`, `SessionStarted`, `SessionEnded`, `PaymentCaptured`

Guidelines:
- Name events in past tense
- Keep them focused on domain meaning
- Emit them from aggregates when state changes matter to the wider system

- **Principles Alignment:** Domain events support loose coupling and help model important business occurrences cleanly.
- **Gotcha:** Events describe completed facts, not commands or intentions.

### Repositories Submodule (`src/domain/repositories/`)
This folder defines repository traits used by the application or infrastructure layer to load and persist aggregates.

Guidelines:
- Define traits only in the domain layer
- Put implementations in infrastructure
- Keep repository contracts focused on domain needs, not database details
- Serperate Create/Read/Update/Delete methods as needed, but avoid overloading with unnecessary operations



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
