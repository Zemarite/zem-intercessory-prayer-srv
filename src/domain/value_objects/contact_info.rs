use crate::domain::enums::contact_method::ContactMethod;
use crate::domain::value_objects::email::Email;

/// Represents contact information as a value object in the domain.
/// Value objects are immutable and compared by value, not identity.
#[derive(Debug, Clone, PartialEq)]
pub struct ContactInfo {
    pub primary_email: Email,
    pub primary_phone: String,
    pub secondary_phone: String,
    pub preferred_contact_method: ContactMethod,
}

impl ContactInfo {
    /// Creates a new ContactInfo value object.
    /// Validates the email format during construction.
    pub fn new(
        primary_email: Email,
        primary_phone: String,
        secondary_phone: String,
        preferred_contact_method: ContactMethod,
    ) -> Self {
        Self {
            primary_email,
            primary_phone,
            secondary_phone,
            preferred_contact_method,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_info_creation() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let contact_info = ContactInfo::new(
            email,
            "+1234567890".to_string(),
            "+0987654321".to_string(),
            ContactMethod::Email,
        );

        assert_eq!(contact_info.primary_email.value, "test@example.com");
        assert_eq!(contact_info.primary_phone, "+1234567890");
        assert_eq!(contact_info.secondary_phone, "+0987654321");
        assert_eq!(contact_info.preferred_contact_method, ContactMethod::Email);
    }

    #[test]
    fn test_contact_info_equality() {
        let email1 = Email::new("test@example.com".to_string()).unwrap();
        let contact_info1 = ContactInfo::new(
            email1,
            "+1234567890".to_string(),
            "+0987654321".to_string(),
            ContactMethod::Email,
        );

        let email2 = Email::new("test@example.com".to_string()).unwrap();
        let contact_info2 = ContactInfo::new(
            email2,
            "+1234567890".to_string(),
            "+0987654321".to_string(),
            ContactMethod::Email,
        );

        let email3 = Email::new("other@example.com".to_string()).unwrap();
        let contact_info3 = ContactInfo::new(
            email3,
            "+1234567890".to_string(),
            "+0987654321".to_string(),
            ContactMethod::Email,
        );

        assert_eq!(contact_info1, contact_info2);
        assert_ne!(contact_info1, contact_info3);
    }

    #[test]
    fn test_contact_info_clone() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let contact_info = ContactInfo::new(
            email,
            "+1234567890".to_string(),
            "+0987654321".to_string(),
            ContactMethod::Phone,
        );

        let cloned = contact_info.clone();
        assert_eq!(contact_info, cloned);
    }

    #[test]
    fn test_contact_info_different_preferred_methods() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let contact_info_email = ContactInfo::new(
            email.clone(),
            "+1234567890".to_string(),
            "+0987654321".to_string(),
            ContactMethod::Email,
        );

        let contact_info_phone = ContactInfo::new(
            email,
            "+1234567890".to_string(),
            "+0987654321".to_string(),
            ContactMethod::Phone,
        );

        assert_ne!(contact_info_email, contact_info_phone);
    }
}