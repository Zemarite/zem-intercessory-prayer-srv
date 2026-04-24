use time::OffsetDateTime;

use crate::domain::enums::{PrayerRequestScope, PrayerRequestStatus};
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{ChurchId, MemberId, OrganizationId, PrayerRequestId};

/// Represents a PrayerRequest entity in the domain.
/// Entities have identity and are compared by their ID, not by value.
#[derive(Debug, Clone)]
pub struct PrayerRequest {
    id: PrayerRequestId,
    title: String,
    description: String,
    is_anonymous: bool,
    status: PrayerRequestStatus,
    scope: PrayerRequestScope,
    church_id: ChurchId,
    organization_id: OrganizationId,
    submitter_member_id: Option<MemberId>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl PrayerRequest {
    /// Creates a new PrayerRequest entity with a generated ID and current timestamps.
    pub fn new(
        title: String,
        description: String,
        is_anonymous: bool,
        scope: PrayerRequestScope,
        church_id: ChurchId,
        organization_id: OrganizationId,
        submitter_member_id: Option<MemberId>,
    ) -> Result<Self, DomainError> {
        if title.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Prayer request title cannot be empty".to_string(),
            ));
        }
        if title.len() > 64 {
            return Err(DomainError::ValidationError(
                "Prayer request title cannot exceed 64 characters".to_string(),
            ));
        }
        if description.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Prayer request description cannot be empty".to_string(),
            ));
        }
        if description.len() > 256 {
            return Err(DomainError::ValidationError(
                "Prayer request description cannot exceed 256 characters".to_string(),
            ));
        }

        let now = OffsetDateTime::now_utc();

        Ok(Self {
            id: PrayerRequestId::generate(),
            title,
            description,
            is_anonymous,
            status: PrayerRequestStatus::Open,
            scope,
            church_id,
            organization_id,
            submitter_member_id,
            created_at: now,
            updated_at: now,
        })
    }

    /// Returns the prayer request's ID.
    pub fn id(&self) -> &PrayerRequestId {
        &self.id
    }

    /// Returns the prayer request's title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the prayer request's description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns whether the prayer request is anonymous.
    pub fn is_anonymous(&self) -> bool {
        self.is_anonymous
    }

    /// Returns the prayer request's status.
    pub fn status(&self) -> &PrayerRequestStatus {
        &self.status
    }

    /// Returns the prayer request's scope.
    pub fn scope(&self) -> &PrayerRequestScope {
        &self.scope
    }

    /// Returns the church ID.
    pub fn church_id(&self) -> &ChurchId {
        &self.church_id
    }

    /// Returns the organization ID.
    pub fn organization_id(&self) -> &OrganizationId {
        &self.organization_id
    }

    /// Returns the submitter member ID.
    pub fn submitter_member_id(&self) -> Option<&MemberId> {
        self.submitter_member_id.as_ref()
    }

    /// Returns the creation timestamp.
    pub fn created_at(&self) -> OffsetDateTime {
        self.created_at
    }

    /// Returns the last update timestamp.
    pub fn updated_at(&self) -> OffsetDateTime {
        self.updated_at
    }

    /// Updates the prayer request's status.
    pub fn update_status(&mut self, status: PrayerRequestStatus) -> Result<(), DomainError> {
        self.status = status;
        self.updated_at = OffsetDateTime::now_utc();
        Ok(())
    }

    /// Updates the prayer request's title.
    pub fn update_title(&mut self, title: String) -> Result<(), DomainError> {
        if title.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Prayer request title cannot be empty".to_string(),
            ));
        }
        if title.len() > 64 {
            return Err(DomainError::ValidationError(
                "Prayer request title cannot exceed 64 characters".to_string(),
            ));
        }
        self.title = title;
        self.updated_at = OffsetDateTime::now_utc();
        Ok(())
    }

    /// Updates the prayer request's description.
    pub fn update_description(&mut self, description: String) -> Result<(), DomainError> {
        if description.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Prayer request description cannot be empty".to_string(),
            ));
        }
        if description.len() > 256 {
            return Err(DomainError::ValidationError(
                "Prayer request description cannot exceed 256 characters".to_string(),
            ));
        }
        self.description = description;
        self.updated_at = OffsetDateTime::now_utc();
        Ok(())
    }
}

impl PartialEq for PrayerRequest {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PrayerRequest {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{ChurchId, OrganizationId};

    #[test]
    fn test_prayer_request_new_valid() {
        let church_id = ChurchId::generate();
        let org_id = OrganizationId::generate();
        let member_id = Some(MemberId::generate());

        let request = PrayerRequest::new(
            "Test Title".to_string(),
            "Test Description".to_string(),
            false,
            PrayerRequestScope::ChurchOnly,
            church_id,
            org_id,
            member_id,
        )
        .unwrap();

        assert_eq!(request.title(), "Test Title");
        assert_eq!(request.description(), "Test Description");
        assert!(!request.is_anonymous());
        assert_eq!(request.status(), &PrayerRequestStatus::Open);
        assert_eq!(request.scope(), &PrayerRequestScope::ChurchOnly);
        assert_eq!(request.church_id(), &church_id);
        assert_eq!(request.organization_id(), &org_id);
        assert_eq!(request.submitter_member_id(), member_id.as_ref());
    }

    #[test]
    fn test_prayer_request_new_empty_title() {
        let church_id = ChurchId::generate();
        let org_id = OrganizationId::generate();

        let result = PrayerRequest::new(
            "".to_string(),
            "Test Description".to_string(),
            false,
            PrayerRequestScope::ChurchOnly,
            church_id,
            org_id,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_prayer_request_new_title_too_long() {
        let church_id = ChurchId::generate();
        let org_id = OrganizationId::generate();
        let long_title = "a".repeat(65);

        let result = PrayerRequest::new(
            long_title,
            "Test Description".to_string(),
            false,
            PrayerRequestScope::ChurchOnly,
            church_id,
            org_id,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_prayer_request_new_empty_description() {
        let church_id = ChurchId::generate();
        let org_id = OrganizationId::generate();

        let result = PrayerRequest::new(
            "Test Title".to_string(),
            "".to_string(),
            false,
            PrayerRequestScope::ChurchOnly,
            church_id,
            org_id,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_prayer_request_new_description_too_long() {
        let church_id = ChurchId::generate();
        let org_id = OrganizationId::generate();
        let long_desc = "a".repeat(257);

        let result = PrayerRequest::new(
            "Test Title".to_string(),
            long_desc,
            false,
            PrayerRequestScope::ChurchOnly,
            church_id,
            org_id,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_prayer_request_update_status() {
        let church_id = ChurchId::generate();
        let org_id = OrganizationId::generate();
        let mut request = PrayerRequest::new(
            "Test Title".to_string(),
            "Test Description".to_string(),
            false,
            PrayerRequestScope::ChurchOnly,
            church_id,
            org_id,
            None,
        )
        .unwrap();

        request
            .update_status(PrayerRequestStatus::Answered)
            .unwrap();
        assert_eq!(request.status(), &PrayerRequestStatus::Answered);
    }

    #[test]
    fn test_prayer_request_update_title() {
        let church_id = ChurchId::generate();
        let org_id = OrganizationId::generate();
        let mut request = PrayerRequest::new(
            "Test Title".to_string(),
            "Test Description".to_string(),
            false,
            PrayerRequestScope::ChurchOnly,
            church_id,
            org_id,
            None,
        )
        .unwrap();

        request.update_title("New Title".to_string()).unwrap();
        assert_eq!(request.title(), "New Title");
    }

    #[test]
    fn test_prayer_request_update_description() {
        let church_id = ChurchId::generate();
        let org_id = OrganizationId::generate();
        let mut request = PrayerRequest::new(
            "Test Title".to_string(),
            "Test Description".to_string(),
            false,
            PrayerRequestScope::ChurchOnly,
            church_id,
            org_id,
            None,
        )
        .unwrap();

        request
            .update_description("New Description".to_string())
            .unwrap();
        assert_eq!(request.description(), "New Description");
    }

    #[test]
    fn test_prayer_request_partial_eq() {
        let church_id = ChurchId::generate();
        let org_id = OrganizationId::generate();
        let request1 = PrayerRequest::new(
            "Title1".to_string(),
            "Desc1".to_string(),
            false,
            PrayerRequestScope::ChurchOnly,
            church_id,
            org_id,
            None,
        )
        .unwrap();
        let request2 = PrayerRequest::new(
            "Title2".to_string(),
            "Desc2".to_string(),
            true,
            PrayerRequestScope::Public,
            church_id,
            org_id,
            None,
        )
        .unwrap();

        // Different IDs, should not be equal
        assert_ne!(request1, request2);

        // Same ID would be equal (but we can't easily test this without exposing internals)
    }
}
