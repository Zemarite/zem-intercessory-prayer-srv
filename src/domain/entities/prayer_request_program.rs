use time::OffsetDateTime;

use crate::domain::value_objects::{PrayerRequestId, PrayerRequestProgramId, ProgramId};

/// Represents a PrayerRequestProgram entity in the domain.
/// This entity manages the assignment of prayer requests to programs.
/// Entities have identity and are compared by their ID, not by value.
#[derive(Debug, Clone)]
pub struct PrayerRequestProgram {
    id: PrayerRequestProgramId,
    prayer_request_id: PrayerRequestId,
    program_id: ProgramId,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl PrayerRequestProgram {
    /// Creates a new PrayerRequestProgram entity with a generated ID and current timestamps.
    pub fn new(
        prayer_request_id: PrayerRequestId,
        program_id: ProgramId,
    ) -> Self {
        let now = OffsetDateTime::now_utc();

        Self {
            id: PrayerRequestProgramId::generate(),
            prayer_request_id,
            program_id,
            created_at: now,
            updated_at: now,
        }
    }

    /// Returns the prayer request program's ID.
    pub fn id(&self) -> &PrayerRequestProgramId {
        &self.id
    }

    /// Returns the prayer request ID.
    pub fn prayer_request_id(&self) -> &PrayerRequestId {
        &self.prayer_request_id
    }

    /// Returns the program ID.
    pub fn program_id(&self) -> &ProgramId {
        &self.program_id
    }

    /// Returns the creation timestamp.
    pub fn created_at(&self) -> OffsetDateTime {
        self.created_at
    }

    /// Returns the last update timestamp.
    pub fn updated_at(&self) -> OffsetDateTime {
        self.updated_at
    }
}

impl PartialEq for PrayerRequestProgram {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PrayerRequestProgram {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{PrayerRequestId, ProgramId};

    #[test]
    fn test_prayer_request_program_new() {
        let prayer_request_id = PrayerRequestId::generate();
        let program_id = ProgramId::generate();

        let assignment = PrayerRequestProgram::new(prayer_request_id, program_id);

        assert_eq!(assignment.prayer_request_id(), &prayer_request_id);
        assert_eq!(assignment.program_id(), &program_id);
        assert_eq!(assignment.created_at(), assignment.updated_at());
    }

    #[test]
    fn test_prayer_request_program_getters() {
        let prayer_request_id = PrayerRequestId::generate();
        let program_id = ProgramId::generate();
        let assignment = PrayerRequestProgram::new(prayer_request_id, program_id);

        assert_eq!(assignment.prayer_request_id(), &prayer_request_id);
        assert_eq!(assignment.program_id(), &program_id);
    }

    #[test]
    fn test_prayer_request_program_equality() {
        let prayer_request_id = PrayerRequestId::generate();
        let program_id = ProgramId::generate();

        let assignment1 = PrayerRequestProgram::new(prayer_request_id, program_id);
        let assignment2 = PrayerRequestProgram::new(prayer_request_id, program_id);

        // Different IDs, so not equal
        assert_ne!(assignment1, assignment2);

        // Same instance should be equal
        let assignment1_clone = assignment1.clone();
        assert_eq!(assignment1, assignment1_clone);
    }
}