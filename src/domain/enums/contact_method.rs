use serde::{Deserialize, Serialize};

/// Enum representing preferred contact methods.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContactMethod {
    Email,
    Phone,
    Sms,
}

impl ContactMethod {
    /// Returns a human-readable string representation of the contact method.
    pub fn as_str(&self) -> &'static str {
        match self {
            ContactMethod::Email => "Email",
            ContactMethod::Phone => "Phone",
            ContactMethod::Sms => "Sms",
        }
    }

    /// Attempts to create a ContactMethod from a string representation.
    /// Case-insensitive matching.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "email" => Some(ContactMethod::Email),
            "phone" => Some(ContactMethod::Phone),
            "sms" | "text" => Some(ContactMethod::Sms),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_method_variants() {
        assert_eq!(ContactMethod::Email, ContactMethod::Email);
        assert_eq!(ContactMethod::Phone, ContactMethod::Phone);
        assert_eq!(ContactMethod::Sms, ContactMethod::Sms);

        assert_ne!(ContactMethod::Email, ContactMethod::Phone);
        assert_ne!(ContactMethod::Phone, ContactMethod::Sms);
        assert_ne!(ContactMethod::Sms, ContactMethod::Email);
    }

    #[test]
    fn test_contact_method_clone() {
        let method = ContactMethod::Email;
        let cloned = method.clone();
        assert_eq!(method, cloned);
    }

    #[test]
    fn test_contact_method_debug() {
        let method = ContactMethod::Email;
        assert_eq!(format!("{:?}", method), "Email");

        let method = ContactMethod::Phone;
        assert_eq!(format!("{:?}", method), "Phone");

        let method = ContactMethod::Sms;
        assert_eq!(format!("{:?}", method), "Sms");
    }

    #[test]
    fn test_contact_method_as_str() {
        assert_eq!(ContactMethod::Email.as_str(), "Email");
        assert_eq!(ContactMethod::Phone.as_str(), "Phone");
        assert_eq!(ContactMethod::Sms.as_str(), "Sms");
    }

    #[test]
    fn test_contact_method_from_str() {
        assert_eq!(ContactMethod::from_str("email"), Some(ContactMethod::Email));
        assert_eq!(ContactMethod::from_str("Email"), Some(ContactMethod::Email));
        assert_eq!(ContactMethod::from_str("phone"), Some(ContactMethod::Phone));
        assert_eq!(ContactMethod::from_str("Phone"), Some(ContactMethod::Phone));
        assert_eq!(ContactMethod::from_str("sms"), Some(ContactMethod::Sms));
        assert_eq!(ContactMethod::from_str("Sms"), Some(ContactMethod::Sms));
        assert_eq!(ContactMethod::from_str("text"), Some(ContactMethod::Sms));
        assert_eq!(ContactMethod::from_str("invalid"), None);
    }
}
