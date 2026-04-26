use async_trait::async_trait;

use crate::domain::entities::prayer_request_program::PrayerRequestProgram;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{PrayerRequestId, PrayerRequestProgramId, ProgramId};

/// Repository trait for PrayerRequestProgram entity operations.
/// This trait defines the contract for data access operations on PrayerRequestProgram aggregates.
#[async_trait]
pub trait PrayerRequestProgramRepository {
    /// Creates a new prayer request program assignment in the repository.
    async fn create(&self, assignment: PrayerRequestProgram) -> Result<(), DomainError>;

    /// Updates an existing prayer request program assignment in the repository.
    async fn update(&self, assignment: &PrayerRequestProgram) -> Result<(), DomainError>;

    /// Finds a prayer request program assignment by its ID.
    async fn find_by_id(
        &self,
        id: &PrayerRequestProgramId,
    ) -> Result<Option<PrayerRequestProgram>, DomainError>;

    /// Finds all program assignments for a specific prayer request.
    async fn find_by_prayer_request(
        &self,
        prayer_request_id: &PrayerRequestId,
    ) -> Result<Vec<PrayerRequestProgram>, DomainError>;

    /// Finds all prayer request assignments for a specific program.
    async fn find_by_program(
        &self,
        program_id: &ProgramId,
    ) -> Result<Vec<PrayerRequestProgram>, DomainError>;

    /// Deletes a prayer request program assignment by its ID.
    async fn delete(&self, id: &PrayerRequestProgramId) -> Result<(), DomainError>;
}
