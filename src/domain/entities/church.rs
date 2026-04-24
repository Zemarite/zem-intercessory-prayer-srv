use crate::domain::errors::DomainError;

use crate::domain::value_objects::{Address, ChurchId, ContactInfo, OrganizationId};
use time::OffsetDateTime;

/// Represents a Church entity in the domain.
/// Entities have identity and are compared by their ID, not by value.
#[derive(Debug, Clone)]
pub struct Church {
    id: ChurchId,
    name: String,
    address: Address,
    contact_info: ContactInfo,
    organization_id: OrganizationId,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl Church {
    /// Creates a new Church entity with a generated ID and current timestamps.
    pub fn new(
        name: String,
        address: Address,
        contact_info: ContactInfo,
        organization_id: OrganizationId,
    ) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Church name cannot be empty".to_string(),
            ));
        }

        let now = OffsetDateTime::now_utc();

        Ok(Self {
            id: ChurchId::generate(),
            name,
            address,
            contact_info,
            organization_id,
            created_at: now,
            updated_at: now,
        })
    }

    // region: Getters for encapsulation
    pub fn id(&self) -> &ChurchId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn contact_info(&self) -> &ContactInfo {
        &self.contact_info
    }

    pub fn organization_id(&self) -> &OrganizationId {
        &self.organization_id
    }

    pub fn created_at(&self) -> &OffsetDateTime {
        &self.created_at
    }

    pub fn updated_at(&self) -> &OffsetDateTime {
        &self.updated_at
    }
    // endregion:Getters for encapsulation

    // Business behavior / methods
    /// Updates the church's name and sets the updated_at timestamp.
    pub fn update_name(&mut self, new_name: String) -> Result<(), DomainError> {
        if new_name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Church name cannot be empty".to_string(),
            ));
        }

        self.name = new_name;
        self.updated_at = OffsetDateTime::now_utc();
        Ok(())
    }

    /// Updates the church's address and sets the updated_at timestamp.
    pub fn update_address(&mut self, new_address: Address) {
        self.address = new_address;
        self.updated_at = OffsetDateTime::now_utc();
    }

    /// Updates the church's contact info and sets the updated_at timestamp.
    pub fn update_contact_info(&mut self, new_contact_info: ContactInfo) {
        self.contact_info = new_contact_info;
        self.updated_at = OffsetDateTime::now_utc();
    }
}

impl PartialEq for Church {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Church {}
