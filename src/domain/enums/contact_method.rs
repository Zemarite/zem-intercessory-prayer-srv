/// Enum representing preferred contact methods.
#[derive(Debug, Clone, PartialEq)]
pub enum ContactMethod {
    Email,
    Phone,
    Sms,
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
}