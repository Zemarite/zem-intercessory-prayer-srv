use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::errors::DomainError;

/// Domain identifier for an organization aggregate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OrganizationId(Uuid);

impl OrganizationId {
    /// Creates an organization identifier from an existing UUID.
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    /// Generates a new version 7 organization identifier.
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    /// Parses an organization identifier from its string representation.
    pub fn parse(value: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(value).map(Self).map_err(|_| {
            DomainError::ValidationError(format!("Invalid organization ID: {value}"))
        })
    }

    /// Returns the wrapped UUID value.
    pub fn value(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for OrganizationId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl From<OrganizationId> for Uuid {
    fn from(value: OrganizationId) -> Self {
        value.value()
    }
}

impl AsRef<Uuid> for OrganizationId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl fmt::Display for OrganizationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_organization_id() {
        let uuid = Uuid::now_v7();
        let organization_id = OrganizationId::parse(&uuid.to_string()).unwrap();

        assert_eq!(organization_id.value(), uuid);
    }

    #[test]
    fn rejects_invalid_organization_id() {
        let error = OrganizationId::parse("not-a-uuid").unwrap_err();

        assert_eq!(
            error,
            DomainError::ValidationError(
                "Invalid organization ID: not-a-uuid".to_string()
            )
        );
    }

    #[test]
    fn generates_distinct_organization_ids() {
        let first = OrganizationId::generate();
        let second = OrganizationId::generate();

        assert_ne!(first, second);
    }
}