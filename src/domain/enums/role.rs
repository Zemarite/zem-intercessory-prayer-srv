/// Enum representing user roles in the domain.
#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Member,
    Intercessor,
    Pastor,
    Admin,
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
}
