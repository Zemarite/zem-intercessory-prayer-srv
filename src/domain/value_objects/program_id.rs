use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::errors::DomainError;

/// Domain identifier for a program aggregate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProgramId(Uuid);

impl ProgramId {
    /// Creates a program identifier from an existing UUID.
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    /// Generates a new version 7 program identifier.
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    /// Parses a program identifier from its string representation.
    pub fn parse(value: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(value)
            .map(Self)
            .map_err(|_| DomainError::ValidationError(format!("Invalid program ID: {value}")))
    }

    /// Returns the wrapped UUID value.
    pub fn value(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for ProgramId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl From<ProgramId> for Uuid {
    fn from(value: ProgramId) -> Self {
        value.value()
    }
}

impl AsRef<Uuid> for ProgramId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl fmt::Display for ProgramId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_program_id() {
        let uuid = Uuid::now_v7();
        let program_id = ProgramId::parse(&uuid.to_string()).unwrap();

        assert_eq!(program_id.value(), uuid);
    }

    #[test]
    fn rejects_invalid_program_id() {
        let error = ProgramId::parse("not-a-uuid").unwrap_err();

        assert_eq!(
            error,
            DomainError::ValidationError("Invalid program ID: not-a-uuid".to_string())
        );
    }

    #[test]
    fn generates_distinct_program_ids() {
        let first = ProgramId::generate();
        let second = ProgramId::generate();

        assert_ne!(first, second);
    }
}
