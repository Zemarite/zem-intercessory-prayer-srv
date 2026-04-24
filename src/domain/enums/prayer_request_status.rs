/// Enum representing prayer request statuses in the domain.
#[derive(Debug, Clone, PartialEq)]
pub enum PrayerRequestStatus {
    Open,
    Answered,
    Closed,
}

impl PrayerRequestStatus {
    /// Returns a human-readable string representation of the prayer request status.
    pub fn as_str(&self) -> &'static str {
        match self {
            PrayerRequestStatus::Open => "Open",
            PrayerRequestStatus::Answered => "Answered",
            PrayerRequestStatus::Closed => "Closed",
        }
    }

    /// Attempts to create a PrayerRequestStatus from a string representation.
    /// Case-insensitive matching.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "open" => Some(PrayerRequestStatus::Open),
            "answered" => Some(PrayerRequestStatus::Answered),
            "closed" => Some(PrayerRequestStatus::Closed),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prayer_request_status_variants() {
        assert_eq!(PrayerRequestStatus::Open, PrayerRequestStatus::Open);
        assert_eq!(PrayerRequestStatus::Answered, PrayerRequestStatus::Answered);
        assert_eq!(PrayerRequestStatus::Closed, PrayerRequestStatus::Closed);

        assert_ne!(PrayerRequestStatus::Open, PrayerRequestStatus::Answered);
        assert_ne!(PrayerRequestStatus::Answered, PrayerRequestStatus::Closed);
    }

    #[test]
    fn test_prayer_request_status_clone() {
        let status = PrayerRequestStatus::Answered;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_prayer_request_status_debug() {
        let status = PrayerRequestStatus::Open;
        assert_eq!(format!("{:?}", status), "Open");

        let status = PrayerRequestStatus::Answered;
        assert_eq!(format!("{:?}", status), "Answered");

        let status = PrayerRequestStatus::Closed;
        assert_eq!(format!("{:?}", status), "Closed");
    }

    #[test]
    fn test_prayer_request_status_as_str() {
        assert_eq!(PrayerRequestStatus::Open.as_str(), "Open");
        assert_eq!(PrayerRequestStatus::Answered.as_str(), "Answered");
        assert_eq!(PrayerRequestStatus::Closed.as_str(), "Closed");
    }

    #[test]
    fn test_prayer_request_status_from_str() {
        assert_eq!(PrayerRequestStatus::from_str("open"), Some(PrayerRequestStatus::Open));
        assert_eq!(PrayerRequestStatus::from_str("Open"), Some(PrayerRequestStatus::Open));
        assert_eq!(PrayerRequestStatus::from_str("answered"), Some(PrayerRequestStatus::Answered));
        assert_eq!(PrayerRequestStatus::from_str("Answered"), Some(PrayerRequestStatus::Answered));
        assert_eq!(PrayerRequestStatus::from_str("closed"), Some(PrayerRequestStatus::Closed));
        assert_eq!(PrayerRequestStatus::from_str("Closed"), Some(PrayerRequestStatus::Closed));
        assert_eq!(PrayerRequestStatus::from_str("invalid"), None);
    }
}