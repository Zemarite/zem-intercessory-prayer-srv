use async_trait::async_trait;

use crate::domain::entities::Church;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{ChurchId, OrganizationId};

/// Repository trait for Church entity operations.
/// This trait defines the contract for data access operations on Church aggregates.
#[async_trait]
pub trait ChurchRepository {
    /// Creates a new church in the repository.
    async fn create(&self, church: Church) -> Result<(), DomainError>;

    /// Updates an existing church in the repository.
    async fn update(&self, church: &Church) -> Result<(), DomainError>;

    /// Finds a church by its ID.
    async fn find_by_id(&self, id: &ChurchId) -> Result<Option<Church>, DomainError>;

    /// Finds all churches belonging to a specific organization.
    async fn list_by_organization_id(
        &self,
        organization_id: &OrganizationId,
    ) -> Result<Vec<Church>, DomainError>;
}
