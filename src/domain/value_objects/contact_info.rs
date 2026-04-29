use crate::domain::enums::ContactMethod;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::email::Email;

/// Represents contact information as a value object in the domain.
/// Value objects are immutable and compared by value, not identity.
#[derive(Debug, Clone, PartialEq)]
pub struct ContactInfo {
    email: Email,
    phone_number: String,
    preferred_contact_method: ContactMethod,
}

impl ContactInfo {
    /// Creates a new ContactInfo value object with validation.
    pub fn new(
        email: Email,
        phone_number: String,
        preferred_contact_method: ContactMethod,
    ) -> Result<Self, DomainError> {
        if phone_number.trim().is_empty() {
            return Err(DomainError::InvalidContactInfo(
                "Phone number cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            email,
            phone_number,
            preferred_contact_method,
        })
    }

    // Getters for encapsulation
    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    pub fn preferred_contact_method(&self) -> &ContactMethod {
        &self.preferred_contact_method
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_info_creation() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let contact_info = ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email)
            .expect("ContactInfo validation failed");

        assert_eq!(contact_info.email().value, "test@example.com");
        assert_eq!(contact_info.phone_number(), "+1234567890");
        assert_eq!(
            contact_info.preferred_contact_method(),
            &ContactMethod::Email
        );
    }

    #[test]
    fn test_contact_info_equality() {
        let email1 = Email::new("test@example.com".to_string()).unwrap();
        let contact_info1 =
            ContactInfo::new(email1, "+1234567890".to_string(), ContactMethod::Email)
                .expect("ContactInfo validation failed");

        let email2 = Email::new("test@example.com".to_string()).unwrap();
        let contact_info2 =
            ContactInfo::new(email2, "+1234567890".to_string(), ContactMethod::Email)
                .expect("ContactInfo validation failed");

        let email3 = Email::new("other@example.com".to_string()).unwrap();
        let contact_info3 =
            ContactInfo::new(email3, "+1234567890".to_string(), ContactMethod::Email)
                .expect("ContactInfo validation failed");

        assert_eq!(contact_info1, contact_info2);
        assert_ne!(contact_info1, contact_info3);
    }

    #[test]
    fn test_contact_info_clone() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let contact_info = ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Phone)
            .expect("ContactInfo validation failed");

        let cloned = contact_info.clone();
        assert_eq!(contact_info, cloned);
    }

    #[test]
    fn test_contact_info_different_preferred_methods() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let contact_info_email = ContactInfo::new(
            email.clone(),
            "+1234567890".to_string(),
            ContactMethod::Email,
        )
        .expect("ContactInfo validation failed");

        let contact_info_phone =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Phone)
                .expect("ContactInfo validation failed");

        assert_ne!(contact_info_email, contact_info_phone);
    }
}
