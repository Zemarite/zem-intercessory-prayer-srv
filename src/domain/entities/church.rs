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
    // pastor_user_id
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::enums::ContactMethod;
    use crate::domain::value_objects::{Address, ContactInfo, Email, OrganizationId};

    #[test]
    fn test_church_creation() {
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        )
        .unwrap();

        let email = Email::new("contact@church.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();

        let organization_id = OrganizationId::generate();

        let church = Church::new(
            "Test Church".to_string(),
            address,
            contact_info,
            organization_id.clone(),
        )
        .unwrap();

        assert_eq!(church.name(), "Test Church");
        assert_eq!(church.organization_id(), &organization_id);
        assert!(church.created_at() <= &OffsetDateTime::now_utc());
        assert!(church.updated_at() <= &OffsetDateTime::now_utc());
        assert_eq!(church.created_at(), church.updated_at());
    }

    #[test]
    fn test_church_creation_fails_with_empty_name() {
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        )
        .unwrap();

        let email = Email::new("contact@church.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();

        let organization_id = OrganizationId::generate();

        let result = Church::new("".to_string(), address, contact_info, organization_id);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::ValidationError("Church name cannot be empty".to_string())
        );
    }

    #[test]
    fn test_church_equality_based_on_id() {
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        )
        .unwrap();

        let email = Email::new("contact@church.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();

        let organization_id = OrganizationId::generate();

        let church1 = Church::new(
            "Church1".to_string(),
            address.clone(),
            contact_info.clone(),
            organization_id.clone(),
        )
        .unwrap();

        let church2 = Church::new(
            "Church2".to_string(),
            address,
            contact_info,
            organization_id,
        )
        .unwrap();

        // Different IDs, so not equal
        assert_ne!(church1, church2);
    }

    #[test]
    fn test_church_update_name() {
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        )
        .unwrap();

        let email = Email::new("contact@church.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();

        let organization_id = OrganizationId::generate();

        let mut church = Church::new(
            "Old Church Name".to_string(),
            address,
            contact_info,
            organization_id,
        )
        .unwrap();

        let old_updated_at = *church.updated_at();

        church.update_name("New Church Name".to_string()).unwrap();

        assert_eq!(church.name(), "New Church Name");
        assert!(church.updated_at() > &old_updated_at);
    }

    #[test]
    fn test_church_update_name_fails_with_empty_name() {
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        )
        .unwrap();

        let email = Email::new("contact@church.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();

        let organization_id = OrganizationId::generate();

        let mut church = Church::new(
            "Test Church".to_string(),
            address,
            contact_info,
            organization_id,
        )
        .unwrap();

        let result = church.update_name("".to_string());

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::ValidationError("Church name cannot be empty".to_string())
        );
    }

    #[test]
    fn test_church_update_address() {
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        )
        .unwrap();

        let email = Email::new("contact@church.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();

        let organization_id = OrganizationId::generate();

        let mut church = Church::new(
            "Test Church".to_string(),
            address,
            contact_info,
            organization_id,
        )
        .unwrap();

        let old_updated_at = *church.updated_at();

        let new_address = Address::new(
            "456 New St".to_string(),
            None,
            "Newtown".to_string(),
            "NY".to_string(),
            "67890".to_string(),
            "USA".to_string(),
        )
        .unwrap();

        church.update_address(new_address.clone());

        assert_eq!(church.address(), &new_address);
        assert!(church.updated_at() > &old_updated_at);
    }

    #[test]
    fn test_church_update_contact_info() {
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        )
        .unwrap();

        let email = Email::new("contact@church.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();

        let organization_id = OrganizationId::generate();

        let mut church = Church::new(
            "Test Church".to_string(),
            address,
            contact_info,
            organization_id,
        )
        .unwrap();

        let old_updated_at = *church.updated_at();

        let new_email = Email::new("newcontact@church.com".to_string()).unwrap();
        let new_contact_info =
            ContactInfo::new(new_email, "+0987654321".to_string(), ContactMethod::Phone).unwrap();

        church.update_contact_info(new_contact_info.clone());

        assert_eq!(church.contact_info(), &new_contact_info);
        assert!(church.updated_at() > &old_updated_at);
    }
}
