use time::OffsetDateTime;

use crate::domain::enums::Role;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{Address, ChurchId, ContactInfo, MemberId};

/// Represents a Member entity in the domain.
/// Entities have identity and are compared by their ID, not by value.
#[derive(Debug, Clone)]
pub struct Member {
    id: MemberId,
    church_id: ChurchId,
    name: String,
    contact_info: ContactInfo,
    address: Address,
    role: Role,
    joined_at: OffsetDateTime,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl Member {
    /// Creates a new Member entity with a generated ID and current timestamps.
    pub fn new(
        church_id: ChurchId,
        name: String,
        contact_info: ContactInfo,
        address: Address,
        role: Role,
    ) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Member name cannot be empty".to_string(),
            ));
        }

        let now = OffsetDateTime::now_utc();

        Ok(Self {
            id: MemberId::generate(),
            church_id,
            name,
            contact_info,
            address,
            role,
            joined_at: now,
            created_at: now,
            updated_at: now,
        })
    }

    // region: Getters for encapsulation
    pub fn id(&self) -> &MemberId {
        &self.id
    }

    pub fn church_id(&self) -> &ChurchId {
        &self.church_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn contact_info(&self) -> &ContactInfo {
        &self.contact_info
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn role(&self) -> &Role {
        &self.role
    }

    pub fn joined_at(&self) -> &OffsetDateTime {
        &self.joined_at
    }

    pub fn created_at(&self) -> &OffsetDateTime {
        &self.created_at
    }

    pub fn updated_at(&self) -> &OffsetDateTime {
        &self.updated_at
    }
    // endregion:Getters for encapsulation

    // Business behavior / methods
    /// Updates the member's name and sets the updated_at timestamp.
    pub fn update_name(&mut self, new_name: String) -> Result<(), DomainError> {
        if new_name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Member name cannot be empty".to_string(),
            ));
        }

        self.name = new_name;
        self.updated_at = OffsetDateTime::now_utc();
        Ok(())
    }

    /// Updates the member's address and sets the updated_at timestamp.
    pub fn update_address(&mut self, new_address: Address) {
        self.address = new_address;
        self.updated_at = OffsetDateTime::now_utc();
    }

    /// Updates the member's contact info and sets the updated_at timestamp.
    pub fn update_contact_info(&mut self, new_contact_info: ContactInfo) {
        self.contact_info = new_contact_info;
        self.updated_at = OffsetDateTime::now_utc();
    }

    /// Updates the member's role and sets the updated_at timestamp.
    pub fn update_role(&mut self, new_role: Role) {
        self.role = new_role;
        self.updated_at = OffsetDateTime::now_utc();
    }
}

impl PartialEq for Member {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Member {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::enums::ContactMethod;
    use crate::domain::value_objects::Email;

    #[test]
    fn test_member_creation() {
        let church_id = ChurchId::generate();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();
        let address = Address::new(
            "123 Main St".to_string(),
            Some("Apt 4".to_string()),
            "Anytown".to_string(),
            "State".to_string(),
            "12345".to_string(),
            "Country".to_string(),
        )
        .unwrap();

        let member = Member::new(
            church_id.clone(),
            "John Doe".to_string(),
            contact_info.clone(),
            address.clone(),
            Role::Member,
        )
        .expect("Member creation should succeed");

        assert_eq!(member.church_id(), &church_id);
        assert_eq!(member.name(), "John Doe");
        assert_eq!(member.contact_info(), &contact_info);
        assert_eq!(member.address(), &address);
        assert_eq!(member.role(), &Role::Member);
        assert!(member.joined_at() <= &OffsetDateTime::now_utc());
        assert!(member.created_at() <= &OffsetDateTime::now_utc());
        assert!(member.updated_at() <= &OffsetDateTime::now_utc());
    }

    #[test]
    fn test_member_creation_fails_with_empty_name() {
        let church_id = ChurchId::generate();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "State".to_string(),
            "12345".to_string(),
            "Country".to_string(),
        )
        .unwrap();

        let result = Member::new(
            church_id,
            "".to_string(),
            contact_info,
            address,
            Role::Member,
        );

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::ValidationError("Member name cannot be empty".to_string())
        );
    }

    #[test]
    fn test_member_equality_based_on_id() {
        let church_id = ChurchId::generate();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let contact_info = ContactInfo::new(
            email.clone(),
            "+1234567890".to_string(),
            ContactMethod::Email,
        )
        .unwrap();
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "State".to_string(),
            "12345".to_string(),
            "Country".to_string(),
        )
        .unwrap();

        let member1 = Member::new(
            church_id.clone(),
            "John Doe".to_string(),
            contact_info.clone(),
            address.clone(),
            Role::Member,
        )
        .unwrap();

        let member2 = Member::new(
            church_id,
            "Jane Doe".to_string(),
            contact_info,
            address,
            Role::Intercessor,
        )
        .unwrap();

        // Different IDs, so not equal
        assert_ne!(member1, member2);

        // Same ID would be equal (though we can't easily test this without cloning)
    }

    #[test]
    fn test_member_update_name() {
        let church_id = ChurchId::generate();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "State".to_string(),
            "12345".to_string(),
            "Country".to_string(),
        )
        .unwrap();

        let mut member = Member::new(
            church_id,
            "John Doe".to_string(),
            contact_info,
            address,
            Role::Member,
        )
        .unwrap();

        let old_updated_at = *member.updated_at();

        member.update_name("Jane Doe".to_string()).unwrap();

        assert_eq!(member.name(), "Jane Doe");
        assert!(member.updated_at() > &old_updated_at);
    }

    #[test]
    fn test_member_update_name_fails_with_empty_name() {
        let church_id = ChurchId::generate();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "State".to_string(),
            "12345".to_string(),
            "Country".to_string(),
        )
        .unwrap();

        let mut member = Member::new(
            church_id,
            "John Doe".to_string(),
            contact_info,
            address,
            Role::Member,
        )
        .unwrap();

        let result = member.update_name("".to_string());

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::ValidationError("Member name cannot be empty".to_string())
        );
    }

    #[test]
    fn test_member_update_address() {
        let church_id = ChurchId::generate();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();
        let address1 = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "State".to_string(),
            "12345".to_string(),
            "Country".to_string(),
        )
        .unwrap();
        let address2 = Address::new(
            "456 Oak Ave".to_string(),
            None,
            "Othertown".to_string(),
            "State".to_string(),
            "67890".to_string(),
            "Country".to_string(),
        )
        .unwrap();

        let mut member = Member::new(
            church_id,
            "John Doe".to_string(),
            contact_info,
            address1,
            Role::Member,
        )
        .unwrap();

        let old_updated_at = *member.updated_at();

        member.update_address(address2.clone());

        assert_eq!(member.address(), &address2);
        assert!(member.updated_at() > &old_updated_at);
    }

    #[test]
    fn test_member_update_contact_info() {
        let church_id = ChurchId::generate();
        let email1 = Email::new("test@example.com".to_string()).unwrap();
        let contact_info1 =
            ContactInfo::new(email1, "+1234567890".to_string(), ContactMethod::Email).unwrap();
        let email2 = Email::new("new@example.com".to_string()).unwrap();
        let contact_info2 =
            ContactInfo::new(email2, "+0987654321".to_string(), ContactMethod::Phone).unwrap();
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "State".to_string(),
            "12345".to_string(),
            "Country".to_string(),
        )
        .unwrap();

        let mut member = Member::new(
            church_id,
            "John Doe".to_string(),
            contact_info1,
            address,
            Role::Member,
        )
        .unwrap();

        let old_updated_at = *member.updated_at();

        member.update_contact_info(contact_info2.clone());

        assert_eq!(member.contact_info(), &contact_info2);
        assert!(member.updated_at() > &old_updated_at);
    }

    #[test]
    fn test_member_update_role() {
        let church_id = ChurchId::generate();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();
        let address = Address::new(
            "123 Main St".to_string(),
            None,
            "Anytown".to_string(),
            "State".to_string(),
            "12345".to_string(),
            "Country".to_string(),
        )
        .unwrap();

        let mut member = Member::new(
            church_id,
            "John Doe".to_string(),
            contact_info,
            address,
            Role::Member,
        )
        .unwrap();

        let old_updated_at = *member.updated_at();

        member.update_role(Role::Pastor);

        assert_eq!(member.role(), &Role::Pastor);
        assert!(member.updated_at() > &old_updated_at);
    }
}
