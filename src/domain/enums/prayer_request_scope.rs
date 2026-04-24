/// Enum representing prayer request scopes in the domain.
#[derive(Debug, Clone, PartialEq)]
pub enum PrayerRequestScope {
    ChurchOnly,
    OrganizationWide,
    GroupOnly,
    Public,
}

impl PrayerRequestScope {
    /// Returns a human-readable string representation of the prayer request scope.
    pub fn as_str(&self) -> &'static str {
        match self {
            PrayerRequestScope::ChurchOnly => "Church Only",
            PrayerRequestScope::OrganizationWide => "Organization Wide",
            PrayerRequestScope::GroupOnly => "Group Only",
            PrayerRequestScope::Public => "Public",
        }
    }

    /// Attempts to create a PrayerRequestScope from a string representation.
    /// Case-insensitive matching.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "church only" | "churchonly" => Some(PrayerRequestScope::ChurchOnly),
            "organization wide" | "organizationwide" => Some(PrayerRequestScope::OrganizationWide),
            "group only" | "grouponly" => Some(PrayerRequestScope::GroupOnly),
            "public" => Some(PrayerRequestScope::Public),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prayer_request_scope_variants() {
        assert_eq!(
            PrayerRequestScope::ChurchOnly,
            PrayerRequestScope::ChurchOnly
        );
        assert_eq!(
            PrayerRequestScope::OrganizationWide,
            PrayerRequestScope::OrganizationWide
        );
        assert_eq!(PrayerRequestScope::GroupOnly, PrayerRequestScope::GroupOnly);
        assert_eq!(PrayerRequestScope::Public, PrayerRequestScope::Public);

        assert_ne!(
            PrayerRequestScope::ChurchOnly,
            PrayerRequestScope::OrganizationWide
        );
        assert_ne!(
            PrayerRequestScope::OrganizationWide,
            PrayerRequestScope::GroupOnly
        );
    }

    #[test]
    fn test_prayer_request_scope_clone() {
        let scope = PrayerRequestScope::Public;
        let cloned = scope.clone();
        assert_eq!(scope, cloned);
    }

    #[test]
    fn test_prayer_request_scope_debug() {
        let scope = PrayerRequestScope::ChurchOnly;
        assert_eq!(format!("{:?}", scope), "ChurchOnly");

        let scope = PrayerRequestScope::OrganizationWide;
        assert_eq!(format!("{:?}", scope), "OrganizationWide");

        let scope = PrayerRequestScope::GroupOnly;
        assert_eq!(format!("{:?}", scope), "GroupOnly");

        let scope = PrayerRequestScope::Public;
        assert_eq!(format!("{:?}", scope), "Public");
    }

    #[test]
    fn test_prayer_request_scope_as_str() {
        assert_eq!(PrayerRequestScope::ChurchOnly.as_str(), "Church Only");
        assert_eq!(
            PrayerRequestScope::OrganizationWide.as_str(),
            "Organization Wide"
        );
        assert_eq!(PrayerRequestScope::GroupOnly.as_str(), "Group Only");
        assert_eq!(PrayerRequestScope::Public.as_str(), "Public");
    }

    #[test]
    fn test_prayer_request_scope_from_str() {
        assert_eq!(
            PrayerRequestScope::from_str("church only"),
            Some(PrayerRequestScope::ChurchOnly)
        );
        assert_eq!(
            PrayerRequestScope::from_str("Church Only"),
            Some(PrayerRequestScope::ChurchOnly)
        );
        assert_eq!(
            PrayerRequestScope::from_str("churchonly"),
            Some(PrayerRequestScope::ChurchOnly)
        );
        assert_eq!(
            PrayerRequestScope::from_str("organization wide"),
            Some(PrayerRequestScope::OrganizationWide)
        );
        assert_eq!(
            PrayerRequestScope::from_str("Organization Wide"),
            Some(PrayerRequestScope::OrganizationWide)
        );
        assert_eq!(
            PrayerRequestScope::from_str("organizationwide"),
            Some(PrayerRequestScope::OrganizationWide)
        );
        assert_eq!(
            PrayerRequestScope::from_str("group only"),
            Some(PrayerRequestScope::GroupOnly)
        );
        assert_eq!(
            PrayerRequestScope::from_str("Group Only"),
            Some(PrayerRequestScope::GroupOnly)
        );
        assert_eq!(
            PrayerRequestScope::from_str("grouponly"),
            Some(PrayerRequestScope::GroupOnly)
        );
        assert_eq!(
            PrayerRequestScope::from_str("public"),
            Some(PrayerRequestScope::Public)
        );
        assert_eq!(
            PrayerRequestScope::from_str("Public"),
            Some(PrayerRequestScope::Public)
        );
        assert_eq!(PrayerRequestScope::from_str("invalid"), None);
    }
}
