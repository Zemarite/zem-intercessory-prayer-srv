use crate::domain::errors::DomainError;
use crate::domain::value_objects::email::Email;

/// Represents billing information as a value object in the domain.
/// Value objects are immutable and compared by value, not identity.
#[derive(Debug, Clone, PartialEq)]
pub struct BillingInfo {
    pub payment_method: String,
    pub billing_email: Email,
    pub tax_id: Option<String>,
}

impl BillingInfo {
    /// Creates a new `BillingInfo` value object with basic validation.
    pub fn new(
        payment_method: String,
        billing_email: Email,
        tax_id: Option<String>,
    ) -> Result<Self, DomainError> {
        let payment_method = payment_method.trim().to_string();
        if payment_method.is_empty() {
            return Err(DomainError::ValidationError(
                "Payment method cannot be empty".to_string(),
            ));
        }

        if payment_method.len() > 50 {
            return Err(DomainError::ValidationError(
                "Payment method is too long (max 50 characters)".to_string(),
            ));
        }

        let normalized_tax_id = tax_id
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        if let Some(ref value) = normalized_tax_id {
            if value.len() > 50 {
                return Err(DomainError::ValidationError(
                    "Tax ID is too long (max 50 characters)".to_string(),
                ));
            }
        }

        Ok(Self {
            payment_method,
            billing_email,
            tax_id: normalized_tax_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billing_info_creation_valid() {
        let billing_email = Email::new("billing@example.com".to_string()).unwrap();

        let billing_info = BillingInfo::new(
            "Credit Card".to_string(),
            billing_email,
            Some("TAX-12345".to_string()),
        )
        .unwrap();

        assert_eq!(billing_info.payment_method, "Credit Card");
        assert_eq!(billing_info.billing_email.value, "billing@example.com");
        assert_eq!(billing_info.tax_id, Some("TAX-12345".to_string()));
    }

    #[test]
    fn test_billing_info_creation_without_tax_id() {
        let billing_email = Email::new("billing@example.com".to_string()).unwrap();

        let billing_info = BillingInfo::new("ACH".to_string(), billing_email, None).unwrap();

        assert_eq!(billing_info.payment_method, "ACH");
        assert_eq!(billing_info.tax_id, None);
    }

    #[test]
    fn test_billing_info_rejects_empty_payment_method() {
        let billing_email = Email::new("billing@example.com".to_string()).unwrap();

        let result = BillingInfo::new("   ".to_string(), billing_email, None);

        assert_eq!(
            result.unwrap_err(),
            DomainError::ValidationError("Payment method cannot be empty".to_string())
        );
    }

    #[test]
    fn test_billing_info_rejects_tax_id_that_is_too_long() {
        let billing_email = Email::new("billing@example.com".to_string()).unwrap();
        let long_tax_id = "T".repeat(51);

        let result = BillingInfo::new(
            "Bank Transfer".to_string(),
            billing_email,
            Some(long_tax_id),
        );

        assert_eq!(
            result.unwrap_err(),
            DomainError::ValidationError("Tax ID is too long (max 50 characters)".to_string())
        );
    }
}
