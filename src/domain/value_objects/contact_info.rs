use crate::domain::enums::contact_method::ContactMethod;
use crate::domain::value_objects::email::Email;
use fluidvalidation::ValidationError;
use fluidvalidation::Validator;

/// Represents contact information as a value object in the domain.
/// Value objects are immutable and compared by value, not identity.
#[derive(Debug, Clone, PartialEq)]
pub struct ContactInfo {
    pub email: Email,
    pub phone_number: String,
    pub preferred_contact_method: ContactMethod,
}

impl ContactInfo {
    /// Creates a new ContactInfo value object.
    /// Validates the email format during construction.
    pub fn new(
        email: Email,
        phone_number: String,
        preferred_contact_method: ContactMethod,
    ) -> Result<Self, Vec<ValidationError>> {
        let validator = contact_info_validator_rules();

        let contact_info = Self {
            email,
            phone_number,
            preferred_contact_method,
        };

        let errors = validator.validate(&contact_info);
        if errors.is_empty() {
            Ok(contact_info)
        } else {
            for error in &errors {
                println!("Validation error: {} - {}", error.path, error.message);
            }
            Err(errors)
        }
    }
}

fn contact_info_validator_rules() -> Validator<ContactInfo> {
    let mut validator = Validator::new();

    validator
        .rule_for("phone_number", |c: &ContactInfo| c.phone_number.as_str())
        .required();

    validator
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
            ContactMethod::Email,
        )
        .expect("ContactInfo validation failed");

        assert_eq!(contact_info.email.value, "test@example.com");
        assert_eq!(contact_info.phone_number, "+1234567890");
        assert_eq!(contact_info.preferred_contact_method, ContactMethod::Email);
    }

    #[test]
    fn test_contact_info_equality() {
        let email1 = Email::new("test@example.com".to_string()).unwrap();
        let contact_info1 = ContactInfo::new(
            email1,
            "+1234567890".to_string(),
            ContactMethod::Email,
        )
        .expect("ContactInfo validation failed");

        let email2 = Email::new("test@example.com".to_string()).unwrap();
        let contact_info2 = ContactInfo::new(
            email2,
            "+1234567890".to_string(),
            ContactMethod::Email,
        )
        .expect("ContactInfo validation failed");

        let email3 = Email::new("other@example.com".to_string()).unwrap();
        let contact_info3 = ContactInfo::new(
            email3,
            "+1234567890".to_string(),
            ContactMethod::Email,
        )
        .expect("ContactInfo validation failed");

        assert_eq!(contact_info1, contact_info2);
        assert_ne!(contact_info1, contact_info3);
    }

    #[test]
    fn test_contact_info_clone() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let contact_info = ContactInfo::new(
            email,
            "+1234567890".to_string(),
            ContactMethod::Phone,
        )
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

        let contact_info_phone = ContactInfo::new(
            email,
            "+1234567890".to_string(),
            ContactMethod::Phone,
        )
        .expect("ContactInfo validation failed");

        assert_ne!(contact_info_email, contact_info_phone);
    }
}
