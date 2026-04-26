use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::errors::DomainError;

/// Domain identifier for a prayer request program assignment aggregate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PrayerRequestProgramId(Uuid);

impl PrayerRequestProgramId {
    /// Creates a prayer request program identifier from an existing UUID.
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    /// Generates a new version 7 prayer request program identifier.
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    /// Parses a prayer request program identifier from its string representation.
    pub fn parse(value: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(value).map(Self).map_err(|_| {
            DomainError::ValidationError(format!("Invalid prayer request program ID: {value}"))
        })
    }

    /// Returns the wrapped UUID value.
    pub fn value(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for PrayerRequestProgramId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl From<PrayerRequestProgramId> for Uuid {
    fn from(value: PrayerRequestProgramId) -> Self {
        value.value()
    }
}

impl AsRef<Uuid> for PrayerRequestProgramId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl fmt::Display for PrayerRequestProgramId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prayer_request_program_id_new() {
        let uuid = Uuid::new_v4();
        let id = PrayerRequestProgramId::new(uuid);
        assert_eq!(id.value(), uuid);
    }

    #[test]
    fn test_prayer_request_program_id_generate() {
        let id1 = PrayerRequestProgramId::generate();
        let id2 = PrayerRequestProgramId::generate();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_prayer_request_program_id_parse_valid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let id = PrayerRequestProgramId::parse(uuid_str).unwrap();
        assert_eq!(id.to_string(), uuid_str);
    }

    #[test]
    fn test_prayer_request_program_id_parse_invalid() {
        let result = PrayerRequestProgramId::parse("invalid-uuid");
        assert!(result.is_err());
    }

    #[test]
    fn test_prayer_request_program_id_display() {
        let uuid = Uuid::new_v4();
        let id = PrayerRequestProgramId::new(uuid);
        assert_eq!(id.to_string(), uuid.to_string());
    }

    #[test]
    fn test_prayer_request_program_id_clone() {
        let id = PrayerRequestProgramId::generate();
        let cloned = id;
        assert_eq!(id, cloned);
    }
}
