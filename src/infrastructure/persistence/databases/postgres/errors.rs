use derive_more::{Display, From};

pub type Result<T> = core::result::Result<T, PersistenceError>;

/// Errors that can occur in the persistence layer.
/// This enum wraps various error types that may arise during database operations.
#[derive(Debug, Display, From)]
#[display("Persistence error: {_variant}")]
pub enum PersistenceError {
    /// Database connection or query errors from SQLx.
    #[from]
    Database(sqlx::Error),
    /// JSON serialization/deserialization errors.
    #[from]
    Serialization(serde_json::Error),
    /// Validation errors for data integrity.
    #[from(String, &str, &String)]
    Validation(String),
}

// region: --- Custom Helpers
impl PersistenceError {
    /// Create a custom error from any type implementing std::error::Error
    pub fn custom_from_err(err: impl std::error::Error) -> Self {
        Self::Validation(err.to_string())
    }

    /// Create a custom error from anything convertible to String
    pub fn custom(val: impl Into<String>) -> Self {
        Self::Validation(val.into())
    }
}
// endregion: --- Custom Helpers

// region: --- Error Boilerplate
impl std::error::Error for PersistenceError {}
// endregion: --- Error Boilerplate
