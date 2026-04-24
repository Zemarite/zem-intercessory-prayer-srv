/// Enum representing user roles in the domain.
#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Member,
    Intercessor,
    Pastor,
    Admin,
}

impl Role {
    /// Returns a human-readable string representation of the role.
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Member => "Member",
            Role::Intercessor => "Intercessor",
            Role::Pastor => "Pastor",
            Role::Admin => "Admin",
        }
    }

    /// Attempts to create a Role from a string representation.
    /// Case-insensitive matching.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "member" => Some(Role::Member),
            "intercessor" => Some(Role::Intercessor),
            "pastor" => Some(Role::Pastor),
            "admin" | "administrator" => Some(Role::Admin),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_variants() {
        assert_eq!(Role::Member, Role::Member);
        assert_eq!(Role::Intercessor, Role::Intercessor);
        assert_eq!(Role::Pastor, Role::Pastor);
        assert_eq!(Role::Admin, Role::Admin);

        assert_ne!(Role::Member, Role::Intercessor);
        assert_ne!(Role::Intercessor, Role::Pastor);
        assert_ne!(Role::Pastor, Role::Admin);
    }

    #[test]
    fn test_role_clone() {
        let role = Role::Pastor;
        let cloned = role.clone();
        assert_eq!(role, cloned);
    }

    #[test]
    fn test_role_debug() {
        let role = Role::Member;
        assert_eq!(format!("{:?}", role), "Member");

        let role = Role::Intercessor;
        assert_eq!(format!("{:?}", role), "Intercessor");

        let role = Role::Pastor;
        assert_eq!(format!("{:?}", role), "Pastor");

        let role = Role::Admin;
        assert_eq!(format!("{:?}", role), "Admin");
    }

    #[test]
    fn test_role_as_str() {
        assert_eq!(Role::Member.as_str(), "Member");
        assert_eq!(Role::Intercessor.as_str(), "Intercessor");
        assert_eq!(Role::Pastor.as_str(), "Pastor");
        assert_eq!(Role::Admin.as_str(), "Admin");
    }

    #[test]
    fn test_role_from_str() {
        assert_eq!(Role::from_str("member"), Some(Role::Member));
        assert_eq!(Role::from_str("Member"), Some(Role::Member));
        assert_eq!(Role::from_str("intercessor"), Some(Role::Intercessor));
        assert_eq!(Role::from_str("Intercessor"), Some(Role::Intercessor));
        assert_eq!(Role::from_str("pastor"), Some(Role::Pastor));
        assert_eq!(Role::from_str("Pastor"), Some(Role::Pastor));
        assert_eq!(Role::from_str("admin"), Some(Role::Admin));
        assert_eq!(Role::from_str("Admin"), Some(Role::Admin));
        assert_eq!(Role::from_str("administrator"), Some(Role::Admin));
        assert_eq!(Role::from_str("invalid"), None);
    }
}
