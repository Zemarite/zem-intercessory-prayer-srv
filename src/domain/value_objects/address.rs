/// Represents an address as a value object in the domain.
/// Value objects are immutable and compared by value, not identity.
#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    pub street_line1: String,
    pub street_line2: Option<String>,
    pub city: String,
    pub state_province: String,
    pub postal_code: String,
    pub country: String,
}

impl Address {
    /// Creates a new Address value object with validation.
    /// Validates that required fields are not empty and have reasonable lengths.
    pub fn new(
        street_line1: String,
        street_line2: Option<String>,
        city: String,
        state_province: String,
        postal_code: String,
        country: String,
    ) -> Result<Self, crate::domain::errors::DomainError> {
        // Validate required fields are not empty
        if street_line1.trim().is_empty() {
            return Err(crate::domain::errors::DomainError::ValidationError(
                "Street line 1 cannot be empty".to_string()
            ));
        }
        if city.trim().is_empty() {
            return Err(crate::domain::errors::DomainError::ValidationError(
                "City cannot be empty".to_string()
            ));
        }
        if state_province.trim().is_empty() {
            return Err(crate::domain::errors::DomainError::ValidationError(
                "State/Province cannot be empty".to_string()
            ));
        }
        if postal_code.trim().is_empty() {
            return Err(crate::domain::errors::DomainError::ValidationError(
                "Postal code cannot be empty".to_string()
            ));
        }
        if country.trim().is_empty() {
            return Err(crate::domain::errors::DomainError::ValidationError(
                "Country cannot be empty".to_string()
            ));
        }

        // Validate reasonable length limits
        if street_line1.len() > 100 {
            return Err(crate::domain::errors::DomainError::ValidationError(
                "Street line 1 is too long (max 100 characters)".to_string()
            ));
        }
        if let Some(ref line2) = street_line2 {
            if line2.len() > 100 {
                return Err(crate::domain::errors::DomainError::ValidationError(
                    "Street line 2 is too long (max 100 characters)".to_string()
                ));
            }
        }
        if city.len() > 50 {
            return Err(crate::domain::errors::DomainError::ValidationError(
                "City is too long (max 50 characters)".to_string()
            ));
        }
        if state_province.len() > 50 {
            return Err(crate::domain::errors::DomainError::ValidationError(
                "State/Province is too long (max 50 characters)".to_string()
            ));
        }
        if postal_code.len() > 20 {
            return Err(crate::domain::errors::DomainError::ValidationError(
                "Postal code is too long (max 20 characters)".to_string()
            ));
        }
        if country.len() > 50 {
            return Err(crate::domain::errors::DomainError::ValidationError(
                "Country is too long (max 50 characters)".to_string()
            ));
        }

        Ok(Self {
            street_line1,
            street_line2,
            city,
            state_province,
            postal_code,
            country,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_creation() {
        let address = Address::new(
            "123 Main St".to_string(),
            Some("Apt 4B".to_string()),
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        );

        assert!(address.is_ok());
        let addr = address.unwrap();
        assert_eq!(addr.street_line1, "123 Main St");
        assert_eq!(addr.street_line2, Some("Apt 4B".to_string()));
        assert_eq!(addr.city, "Anytown");
        assert_eq!(addr.state_province, "CA");
        assert_eq!(addr.postal_code, "12345");
        assert_eq!(addr.country, "USA");
    }

    #[test]
    fn test_address_validation_empty_fields() {
        // Test empty street_line1
        assert!(Address::new(
            "".to_string(),
            Some("Apt 4B".to_string()),
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        ).is_err());

        // Test empty city
        assert!(Address::new(
            "123 Main St".to_string(),
            Some("Apt 4B".to_string()),
            "".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        ).is_err());

        // Test empty state_province
        assert!(Address::new(
            "123 Main St".to_string(),
            Some("Apt 4B".to_string()),
            "Anytown".to_string(),
            "".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        ).is_err());

        // Test empty postal_code
        assert!(Address::new(
            "123 Main St".to_string(),
            Some("Apt 4B".to_string()),
            "Anytown".to_string(),
            "CA".to_string(),
            "".to_string(),
            "USA".to_string(),
        ).is_err());

        // Test empty country
        assert!(Address::new(
            "123 Main St".to_string(),
            Some("Apt 4B".to_string()),
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "".to_string(),
        ).is_err());
    }

    #[test]
    fn test_address_validation_whitespace_only() {
        // Test whitespace-only street_line1
        assert!(Address::new(
            "   ".to_string(),
            Some("Apt 4B".to_string()),
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        ).is_err());
    }

    #[test]
    fn test_address_validation_length_limits() {
        // Test street_line1 too long
        let long_street = "a".repeat(101);
        assert!(Address::new(
            long_street,
            Some("Apt 4B".to_string()),
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        ).is_err());

        // Test street_line2 too long
        let long_street2 = "b".repeat(101);
        assert!(Address::new(
            "123 Main St".to_string(),
            Some(long_street2),
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        ).is_err());

        // Test city too long
        let long_city = "c".repeat(51);
        assert!(Address::new(
            "123 Main St".to_string(),
            Some("Apt 4B".to_string()),
            long_city,
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        ).is_err());

        // Test postal_code too long
        let long_postal = "d".repeat(21);
        assert!(Address::new(
            "123 Main St".to_string(),
            Some("Apt 4B".to_string()),
            "Anytown".to_string(),
            "CA".to_string(),
            long_postal,
            "USA".to_string(),
        ).is_err());
    }

    #[test]
    fn test_address_equality() {
        let address1 = Address::new(
            "123 Main St".to_string(),
            Some("Apt 4B".to_string()),
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        ).unwrap();

        let address2 = Address::new(
            "123 Main St".to_string(),
            Some("Apt 4B".to_string()),
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        ).unwrap();

        let address3 = Address::new(
            "456 Oak St".to_string(),
            Some("Apt 4B".to_string()),
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        ).unwrap();

        assert_eq!(address1, address2);
        assert_ne!(address1, address3);
    }

    #[test]
    fn test_address_clone() {
        let address = Address::new(
            "123 Main St".to_string(),
            Some("Apt 4B".to_string()),
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        ).unwrap();

        let cloned = address.clone();
        assert_eq!(address, cloned);
    }
}
