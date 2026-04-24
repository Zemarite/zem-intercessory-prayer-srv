use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::errors::DomainError;

/// Domain identifier for a member aggregate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MemberId(Uuid);

impl MemberId {
    /// Creates a user identifier from an existing UUID.
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    /// Generates a new version 7 user identifier.
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    /// Parses a user identifier from its string representation.
    pub fn parse(value: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(value)
            .map(Self)
            .map_err(|_| DomainError::ValidationError(format!("Invalid user ID: {value}")))
    }

    /// Returns the wrapped UUID value.
    pub fn value(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for MemberId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl From<MemberId> for Uuid {
    fn from(value: MemberId) -> Self {
        value.value()
    }
}

impl AsRef<Uuid> for MemberId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl fmt::Display for MemberId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_user_id() {
        let uuid = Uuid::now_v7();
        let user_id = MemberId::parse(&uuid.to_string()).unwrap();

        assert_eq!(user_id.value(), uuid);
    }

    #[test]
    fn rejects_invalid_user_id() {
        let error = MemberId::parse("not-a-uuid").unwrap_err();

        assert_eq!(
            error,
            DomainError::ValidationError("Invalid user ID: not-a-uuid".to_string())
        );
    }

    #[test]
    fn generates_distinct_user_ids() {
        let first = MemberId::generate();
        let second = MemberId::generate();

        assert_ne!(first, second);
    }
}
