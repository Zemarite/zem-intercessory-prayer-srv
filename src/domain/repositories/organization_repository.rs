use async_trait::async_trait;

use crate::domain::entities::Organization;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::OrganizationId;

/// Repository trait for Organization entity operations.
/// This trait defines the contract for data access operations on Organization aggregates.
#[async_trait]
pub trait OrganizationRepository {
    /// Creates a new organization in the repository.
    async fn create(&self, organization: Organization) -> Result<(), DomainError>;

    /// Updates an existing organization in the repository.
    async fn update(&self, organization: &Organization) -> Result<(), DomainError>;

    /// Finds an organization by its ID.
    async fn find_by_id(&self, id: &OrganizationId) -> Result<Option<Organization>, DomainError>;
}
