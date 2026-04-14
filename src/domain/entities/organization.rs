use time::OffsetDateTime;

use crate::domain::errors::DomainError;
use crate::domain::value_objects::address::Address;
use crate::domain::value_objects::billing_info::BillingInfo;
use crate::domain::value_objects::contact_info::ContactInfo;
use crate::domain::value_objects::organization_id::OrganizationId;

/// Represents an Organization entity in the domain.
/// Entities have identity and are compared by their ID, not by value.
#[derive(Debug, Clone)]
pub struct Organization {
    pub id: OrganizationId,
    pub name: String,
    pub address: Address,
    pub contact_info: ContactInfo,
    pub billing: Option<BillingInfo>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Organization {
    /// Creates a new Organization entity with a generated ID and current timestamps.
    pub fn new(
        name: String,
        address: Address,
        contact_info: ContactInfo,
        billing: Option<BillingInfo>,
    ) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Organization name cannot be empty".to_string(),
            ));
        }

        let now = OffsetDateTime::now_utc();

        Ok(Self {
            id: OrganizationId::generate(),
            name,
            address,
            contact_info,
            billing,
            created_at: now,
            updated_at: now,
        })
    }

    // Business behavior / methods
    /// Updates the organization's name and sets the updated_at timestamp.
    pub fn update_name(&mut self, new_name: String) -> Result<(), DomainError> {
        if new_name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Organization name cannot be empty".to_string(),
            ));
        }

        self.name = new_name;
        self.updated_at = OffsetDateTime::now_utc();
        Ok(())
    }

    /// Updates the organization's address and sets the updated_at timestamp.
    pub fn update_address(&mut self, new_address: Address) {
        self.address = new_address;
        self.updated_at = OffsetDateTime::now_utc();
    }

    /// Updates the organization's contact info and sets the updated_at timestamp.
    pub fn update_contact_info(&mut self, new_contact_info: ContactInfo) {
        self.contact_info = new_contact_info;
        self.updated_at = OffsetDateTime::now_utc();
    }

    /// Updates the organization's billing info and sets the updated_at timestamp.
    pub fn update_billing(&mut self, new_billing: Option<BillingInfo>) {
        self.billing = new_billing;
        self.updated_at = OffsetDateTime::now_utc();
    }
}

impl PartialEq for Organization {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Organization {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::enums::contact_method::ContactMethod;
    use crate::domain::enums::payment_method::PaymentMethod;
    use crate::domain::value_objects::email::Email;

    #[test]
    fn test_organization_creation() {
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        )
        .unwrap();

        let email = Email::new("contact@example.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();

        let billing_email = Email::new("billing@example.com".to_string()).unwrap();
        let billing =
            Some(BillingInfo::new(PaymentMethod::CreditCard, billing_email, None).unwrap());

        let org = Organization::new(
            "Test Organization".to_string(),
            address,
            contact_info,
            billing,
        )
        .unwrap();

        assert_eq!(org.name, "Test Organization");
        assert!(org.billing.is_some());
        assert_eq!(org.created_at, org.updated_at);
    }

    #[test]
    fn test_organization_creation_fails_with_empty_name() {
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        )
        .unwrap();

        let email = Email::new("contact@example.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();

        let result = Organization::new("".to_string(), address, contact_info, None);

        assert!(result.is_err());
    }

    #[test]
    fn test_organization_equality_based_on_id() {
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        )
        .unwrap();

        let email = Email::new("contact@example.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();

        let org1 = Organization::new(
            "Org1".to_string(),
            address.clone(),
            contact_info.clone(),
            None,
        )
        .unwrap();

        let org2 = Organization::new("Org2".to_string(), address, contact_info, None).unwrap();

        // Different IDs, so not equal even with same other fields
        assert_ne!(org1, org2);

        // Same ID would be equal (though hard to test without cloning)
    }

    #[test]
    fn test_organization_update_name() {
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        )
        .unwrap();

        let email = Email::new("contact@example.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();

        let mut org =
            Organization::new("Old Name".to_string(), address, contact_info, None).unwrap();

        let old_updated_at = org.updated_at;

        org.update_name("New Name".to_string()).unwrap();

        assert_eq!(org.name, "New Name");
        assert!(org.updated_at > old_updated_at);
    }
}
