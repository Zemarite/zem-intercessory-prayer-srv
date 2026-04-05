/// Enum representing different payment methods available for billing.
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethod {
    CreditCard,
    DebitCard,
    BankTransfer,
    PayPal,
    ApplePay,
    GooglePay,
    Cash,
    Check,
}

impl PaymentMethod {
    /// Returns a human-readable string representation of the payment method.
    pub fn as_str(&self) -> &'static str {
        match self {
            PaymentMethod::CreditCard => "Credit Card",
            PaymentMethod::DebitCard => "Debit Card",
            PaymentMethod::BankTransfer => "Bank Transfer",
            PaymentMethod::PayPal => "PayPal",
            PaymentMethod::ApplePay => "Apple Pay",
            PaymentMethod::GooglePay => "Google Pay",
            PaymentMethod::Cash => "Cash",
            PaymentMethod::Check => "Check",
        }
    }

    /// Attempts to create a PaymentMethod from a string representation.
    /// Case-insensitive matching.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "credit card" | "creditcard" => Some(PaymentMethod::CreditCard),
            "debit card" | "debitcard" => Some(PaymentMethod::DebitCard),
            "bank transfer" | "banktransfer" | "ach" => Some(PaymentMethod::BankTransfer),
            "paypal" => Some(PaymentMethod::PayPal),
            "apple pay" | "applepay" => Some(PaymentMethod::ApplePay),
            "google pay" | "googlepay" => Some(PaymentMethod::GooglePay),
            "cash" => Some(PaymentMethod::Cash),
            "check" => Some(PaymentMethod::Check),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_method_variants() {
        assert_eq!(PaymentMethod::CreditCard, PaymentMethod::CreditCard);
        assert_eq!(PaymentMethod::DebitCard, PaymentMethod::DebitCard);
        assert_eq!(PaymentMethod::BankTransfer, PaymentMethod::BankTransfer);
        assert_eq!(PaymentMethod::PayPal, PaymentMethod::PayPal);
        assert_eq!(PaymentMethod::ApplePay, PaymentMethod::ApplePay);
        assert_eq!(PaymentMethod::GooglePay, PaymentMethod::GooglePay);
        assert_eq!(PaymentMethod::Cash, PaymentMethod::Cash);
        assert_eq!(PaymentMethod::Check, PaymentMethod::Check);
    }

    #[test]
    fn test_payment_method_as_str() {
        assert_eq!(PaymentMethod::CreditCard.as_str(), "Credit Card");
        assert_eq!(PaymentMethod::DebitCard.as_str(), "Debit Card");
        assert_eq!(PaymentMethod::BankTransfer.as_str(), "Bank Transfer");
        assert_eq!(PaymentMethod::PayPal.as_str(), "PayPal");
        assert_eq!(PaymentMethod::ApplePay.as_str(), "Apple Pay");
        assert_eq!(PaymentMethod::GooglePay.as_str(), "Google Pay");
        assert_eq!(PaymentMethod::Cash.as_str(), "Cash");
        assert_eq!(PaymentMethod::Check.as_str(), "Check");
    }

    #[test]
    fn test_payment_method_from_str() {
        assert_eq!(
            PaymentMethod::from_str("Credit Card"),
            Some(PaymentMethod::CreditCard)
        );
        assert_eq!(
            PaymentMethod::from_str("creditcard"),
            Some(PaymentMethod::CreditCard)
        );
        assert_eq!(
            PaymentMethod::from_str("debit card"),
            Some(PaymentMethod::DebitCard)
        );
        assert_eq!(
            PaymentMethod::from_str("bank transfer"),
            Some(PaymentMethod::BankTransfer)
        );
        assert_eq!(
            PaymentMethod::from_str("ACH"),
            Some(PaymentMethod::BankTransfer)
        );
        assert_eq!(
            PaymentMethod::from_str("paypal"),
            Some(PaymentMethod::PayPal)
        );
        assert_eq!(
            PaymentMethod::from_str("apple pay"),
            Some(PaymentMethod::ApplePay)
        );
        assert_eq!(
            PaymentMethod::from_str("google pay"),
            Some(PaymentMethod::GooglePay)
        );
        assert_eq!(PaymentMethod::from_str("cash"), Some(PaymentMethod::Cash));
        assert_eq!(PaymentMethod::from_str("check"), Some(PaymentMethod::Check));
        assert_eq!(PaymentMethod::from_str("invalid"), None);
    }

    #[test]
    fn test_payment_method_clone() {
        let method = PaymentMethod::CreditCard;
        let cloned = method.clone();
        assert_eq!(method, cloned);
    }

    #[test]
    fn test_payment_method_debug() {
        let method = PaymentMethod::CreditCard;
        assert_eq!(format!("{:?}", method), "CreditCard");
    }
}
