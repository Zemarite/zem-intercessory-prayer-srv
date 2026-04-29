use async_trait::async_trait;

use crate::domain::entities::PrayerRequest;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{ChurchId, OrganizationId, PrayerRequestId};

/// Repository trait for PrayerRequest entity operations.
/// This trait defines the contract for data access operations on PrayerRequest aggregates.
#[async_trait]
pub trait PrayerRequestRepository {
    /// Creates a new prayer request in the repository.
    async fn create(&self, prayer_request: PrayerRequest) -> Result<(), DomainError>;

    /// Updates an existing prayer request in the repository.
    async fn update(&self, prayer_request: &PrayerRequest) -> Result<(), DomainError>;

    /// Finds a prayer request by its ID.
    async fn find_by_id(&self, id: &PrayerRequestId) -> Result<Option<PrayerRequest>, DomainError>;

    /// Finds all prayer requests belonging to a specific church.
    async fn find_by_church_id(
        &self,
        church_id: &ChurchId,
    ) -> Result<Vec<PrayerRequest>, DomainError>;

    /// Finds all prayer requests belonging to a specific organization.
    async fn find_by_organization_id(
        &self,
        organization_id: &OrganizationId,
    ) -> Result<Vec<PrayerRequest>, DomainError>;
}
