use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::errors::DomainError;

/// Domain identifier for a church aggregate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChurchId(Uuid);

impl ChurchId {
    /// Creates a church identifier from an existing UUID.
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    /// Generates a new version 7 church identifier.
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    /// Parses a church identifier from its string representation.
    pub fn parse(value: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(value)
            .map(Self)
            .map_err(|_| DomainError::ValidationError(format!("Invalid church ID: {value}")))
    }

    /// Returns the wrapped UUID value.
    pub fn value(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for ChurchId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl From<ChurchId> for Uuid {
    fn from(value: ChurchId) -> Self {
        value.value()
    }
}

impl AsRef<Uuid> for ChurchId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl fmt::Display for ChurchId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_church_id() {
        let uuid = Uuid::now_v7();
        let church_id = ChurchId::parse(&uuid.to_string()).unwrap();

        assert_eq!(church_id.value(), uuid);
    }

    #[test]
    fn rejects_invalid_church_id() {
        let error = ChurchId::parse("not-a-uuid").unwrap_err();

        assert_eq!(
            error,
            DomainError::ValidationError("Invalid church ID: not-a-uuid".to_string())
        );
    }

    #[test]
    fn generates_distinct_church_ids() {
        let first = ChurchId::generate();
        let second = ChurchId::generate();

        assert_ne!(first, second);
    }
}
