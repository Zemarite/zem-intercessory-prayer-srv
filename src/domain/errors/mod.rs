use thiserror::Error;

/// Domain-level errors for the application.
/// These represent business logic violations and validation errors.
#[derive(Error, Debug, PartialEq)]
pub enum DomainError {
    #[error("Invalid email format: {0}")]
    InvalidEmail(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}
