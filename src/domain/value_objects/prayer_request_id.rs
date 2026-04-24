use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::errors::DomainError;

/// Domain identifier for a prayer request aggregate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PrayerRequestId(Uuid);

impl PrayerRequestId {
    /// Creates a prayer request identifier from an existing UUID.
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    /// Generates a new version 7 prayer request identifier.
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    /// Parses a prayer request identifier from its string representation.
    pub fn parse(value: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(value).map(Self).map_err(|_| {
            DomainError::ValidationError(format!("Invalid prayer request ID: {value}"))
        })
    }

    /// Returns the wrapped UUID value.
    pub fn value(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for PrayerRequestId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl From<PrayerRequestId> for Uuid {
    fn from(value: PrayerRequestId) -> Self {
        value.value()
    }
}

impl AsRef<Uuid> for PrayerRequestId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl fmt::Display for PrayerRequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prayer_request_id_new() {
        let uuid = Uuid::new_v4();
        let id = PrayerRequestId::new(uuid);
        assert_eq!(id.value(), uuid);
    }

    #[test]
    fn test_prayer_request_id_generate() {
        let id1 = PrayerRequestId::generate();
        let id2 = PrayerRequestId::generate();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_prayer_request_id_parse_valid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let id = PrayerRequestId::parse(uuid_str).unwrap();
        assert_eq!(id.to_string(), uuid_str);
    }

    #[test]
    fn test_prayer_request_id_parse_invalid() {
        let result = PrayerRequestId::parse("invalid-uuid");
        assert!(result.is_err());
    }

    #[test]
    fn test_prayer_request_id_display() {
        let uuid = Uuid::new_v4();
        let id = PrayerRequestId::new(uuid);
        assert_eq!(id.to_string(), uuid.to_string());
    }

    #[test]
    fn test_prayer_request_id_clone() {
        let id = PrayerRequestId::generate();
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    #[test]
    fn test_prayer_request_id_partial_eq() {
        let id1 = PrayerRequestId::generate();
        let id2 = id1;
        assert_eq!(id1, id2);
    }
}
