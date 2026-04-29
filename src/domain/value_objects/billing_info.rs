use crate::domain::enums::PaymentMethod;
use crate::domain::errors::{DomainError, Result};
use crate::domain::value_objects::email::Email;
use serde::{Deserialize, Serialize};

/// Represents billing information as a value object in the domain.
/// Value objects are immutable and compared by value, not identity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BillingInfo {
    payment_method: PaymentMethod,
    billing_email: Email,
    tax_id: Option<String>,
}

impl BillingInfo {
    /// Creates a new `BillingInfo` value object with basic validation.
    pub fn new(
        payment_method: PaymentMethod,
        billing_email: Email,
        tax_id: Option<String>,
    ) -> Result<Self> {
        let normalized_tax_id = tax_id
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        if let Some(ref value) = normalized_tax_id {
            if value.len() > 50 {
                return Err(DomainError::InvalidBillingInfo(
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

    /// Creates a new `BillingInfo` from a payment method string.
    /// Attempts to parse the string into a PaymentMethod enum.
    pub fn from_payment_method_str(
        payment_method_str: &str,
        billing_email: Email,
        tax_id: Option<String>,
    ) -> Result<Self> {
        let payment_method = PaymentMethod::from_str(payment_method_str).ok_or_else(|| {
            DomainError::InvalidBillingInfo(format!(
                "Invalid payment method: {}",
                payment_method_str
            ))
        })?;

        Self::new(payment_method, billing_email, tax_id)
    }

    // Getters
    pub fn payment_method(&self) -> &PaymentMethod {
        &self.payment_method
    }

    pub fn billing_email(&self) -> &Email {
        &self.billing_email
    }

    pub fn tax_id(&self) -> Option<&String> {
        self.tax_id.as_ref()
    }

    // Business behavior / methods
    pub fn has_tax_id(&self) -> bool {
        self.tax_id.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billing_info_creation_valid() {
        let billing_email = Email::new("billing@example.com".to_string()).unwrap();

        let billing_info = BillingInfo::new(
            PaymentMethod::CreditCard,
            billing_email,
            Some("TAX-12345".to_string()),
        )
        .unwrap();

        assert_eq!(*billing_info.payment_method(), PaymentMethod::CreditCard);
        assert_eq!(billing_info.billing_email().value, "billing@example.com");
        assert_eq!(billing_info.tax_id(), Some(&"TAX-12345".to_string()));
    }

    #[test]
    fn test_billing_info_creation_without_tax_id() {
        let billing_email = Email::new("billing@example.com".to_string()).unwrap();

        let billing_info =
            BillingInfo::new(PaymentMethod::BankTransfer, billing_email, None).unwrap();

        assert_eq!(*billing_info.payment_method(), PaymentMethod::BankTransfer);
        assert_eq!(billing_info.tax_id(), None);
    }

    #[test]
    fn test_billing_info_rejects_tax_id_that_is_too_long() {
        let billing_email = Email::new("billing@example.com".to_string()).unwrap();
        let long_tax_id = "T".repeat(51);

        let result = BillingInfo::new(PaymentMethod::PayPal, billing_email, Some(long_tax_id));

        // assert!(
        //     matches!(result.unwrap_err(),
        //     DomainError::InvalidBillingInfo(ref msg) if msg == "Tax ID is too long (max 50 characters)")
        // );
        assert_eq!(
            result.unwrap_err(),
            DomainError::InvalidBillingInfo("Tax ID is too long (max 50 characters)".to_string())
        );
    }

    #[test]
    fn test_billing_info_from_payment_method_str() {
        let billing_email = Email::new("billing@example.com".to_string()).unwrap();

        let billing_info = BillingInfo::from_payment_method_str(
            "credit card",
            billing_email,
            Some("TAX-12345".to_string()),
        )
        .unwrap();

        assert_eq!(*billing_info.payment_method(), PaymentMethod::CreditCard);
        assert_eq!(billing_info.billing_email().value, "billing@example.com");
        assert_eq!(billing_info.tax_id(), Some(&"TAX-12345".to_string()));
    }

    #[test]
    fn test_billing_info_from_invalid_payment_method_str() {
        let billing_email = Email::new("billing@example.com".to_string()).unwrap();

        let result = BillingInfo::from_payment_method_str("invalid method", billing_email, None);
        assert_eq!(
            result.unwrap_err(),
            DomainError::InvalidBillingInfo("Invalid payment method: invalid method".to_string())
        );
    }
}
