use async_trait::async_trait;

use crate::domain::entities::program::Program;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{ChurchId, ProgramId};

/// Repository trait for Program entity operations.
/// This trait defines the contract for data access operations on Program aggregates.
#[async_trait]
pub trait ProgramRepository {
    /// Creates a new program in the repository.
    async fn create(&self, program: Program) -> Result<(), DomainError>;

    /// Updates an existing program in the repository.
    async fn update(&self, program: &Program) -> Result<(), DomainError>;

    /// Finds a program by its ID.
    async fn find_by_id(&self, id: &ProgramId) -> Result<Option<Program>, DomainError>;

    /// Finds all programs belonging to a specific church.
    async fn list_by_church_id(&self, church_id: &ChurchId) -> Result<Vec<Program>, DomainError>;

    /// Finds programs by program type within a church.
    async fn list_by_church_and_type(
        &self,
        church_id: &ChurchId,
        program_type: &crate::domain::enums::ProgramType,
    ) -> Result<Vec<Program>, DomainError>;

    /// Finds programs that are currently active (no end date or end date in future).
    async fn list_active_by_church_id(&self, church_id: &ChurchId) -> Result<Vec<Program>, DomainError>;
}