---
name: rust-clean-arch-errors-rs
description: Generates or updates errors.rs files for any layer in a Clean Architecture Rust project (Domain, Application, Infrastructure, Interface/Presentation). Reviews the existing code in the target layer/module, creates a consistent errors.rs using the provided template if missing, or intelligently extends an existing one while preserving custom variants and maintaining layer-appropriate error handling.
argument-hint: /errors-rs [layer] [module-name]
user-invocable: true
---

# Rust Clean Architecture — errors.rs Generator Skill

**Purpose**  
This skill ensures consistent, idiomatic, and maintainable error handling across all layers of a **Clean Architecture** (or Hexagonal/Onion) Rust project by generating or updating `errors.rs` files.

It enforces the **dependency rule** while providing a uniform error pattern using `derive_more` for `Display` + `From` implementations, a custom `Result` type alias, and helper methods for ergonomic error creation.

## When to Use This Skill

Use this skill when the user:

- Asks to create error handling for a new module or layer in a Clean Architecture project.
- Wants to standardize or improve existing error definitions.
- Adds new external dependencies (e.g., database, HTTP client, serialization) that require error variants.
- Requests a review of error handling in Domain, Application, Infrastructure, or Interface layers.
- Needs to optimize triggering accuracy or performance of error-related code generation.

**Supported Layers** (map to typical directory structure):

- **Domain** — Pure business errors (no external crates except core). Minimal dependencies.
- **Application** — Use case / service errors. Can wrap Domain errors + application-specific logic.
- **Infrastructure** — Concrete implementations (DB, external APIs, file I/O, etc.). Wraps external crate errors heavily.
- **Interface** (or Presentation / API) — Adapter layer errors (e.g., HTTP response mapping, serialization). Often converts inner errors to user-facing responses.

## Skill Behavior & Procedure

1. **Review the Layer**
    - Analyze all Rust files in the specified layer/module.
    - Identify existing error types, external dependencies (e.g., `std::io::Error`, `sqlx::Error`, `serde_json::Error`, `axum::http::StatusCode`, etc.).
    - Detect any current `errors.rs` or inline error enums.

2. **Decision Logic**
    - **If no `errors.rs` exists** → Create a new one using the base template, tailored to the layer.
    - **If `errors.rs` exists** →
        - Preserve all existing custom variants.
        - Add missing `#[from]` conversions for newly detected external errors.
        - Ensure `Result<T>` type alias and boilerplate are present.
        - Add or update custom helper methods if needed.
        - Maintain clean region comments for readability.

3. **Layer-Specific Guidelines**
    - **Domain Layer**:
        - Keep errors pure (avoid external crate dependencies when possible).
        - Prefer business-oriented variants (`InvalidInput`, `NotFound`, `BusinessRuleViolation`).
        - Use `Custom(String)` sparingly.

    - **Application Layer**:
        - Can derive `From` for `Domain::Error`.
        - Add use-case specific errors (e.g., `ValidationFailed`, `OperationNotAllowed`).

    - **Infrastructure Layer**:
        - Aggressively add `#[from]` for external errors (DB, network, parsing, etc.).
        - Provide mapping to inner domain/application errors where appropriate.

    - **Interface / Presentation Layer**:
        - Focus on conversion to response types (e.g., `IntoResponse` for Axum).
        - May include HTTP status mapping logic.

4. **Output Format**
    - Always output the complete, ready-to-use `errors.rs` content.
    - Include a short explanation of changes made.
    - Suggest follow-up steps (e.g., “Add `From<YourLayerError> for ApplicationError` in the Application layer”).

## Base Code Template (Customized per Layer)

```rust
use derive_more::{Display, From};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Display, From)]
#[display("{self:?}")]
pub enum Error {
    #[from(String, &String, &str)]
    Custom(String),

    // -- Domain Errors (for Application/Infrastructure/Interface layers)
    // #[from]
    // Domain(crate::domain::Error),

    // -- Externals (add per layer needs)
    // #[from]
    // Io(std::io::Error),
}

/// region: --- Custom Helpers
impl Error {
    /// Create a custom error from any type implementing std::error::Error
    pub fn custom_from_err(err: impl std::error::Error) -> Self {
        Self::Custom(err.to_string())
    }

    /// Create a custom error from anything convertible to String
    pub fn custom(val: impl Into<String>) -> Self {
        Self::Custom(val.into())
    }
}
// endregion: --- Custom Helpers

// region: --- Error Boilerplate
impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
```
