use async_trait::async_trait;

use crate::domain::entities::member::Member;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{ChurchId, MemberId};

/// Repository trait for Member entity operations.
/// This trait defines the contract for data access operations on Member aggregates.
#[async_trait]
pub trait MemberRepository {
    /// Creates a new member in the repository.
    async fn create(&self, member: Member) -> Result<(), DomainError>;

    /// Updates an existing member in the repository.
    async fn update(&self, member: &Member) -> Result<(), DomainError>;

    /// Finds a member by its ID.
    async fn find_by_id(&self, id: &MemberId) -> Result<Option<Member>, DomainError>;

    /// Finds all members belonging to a specific church.
    async fn list_by_church_id(&self, church_id: &ChurchId) -> Result<Vec<Member>, DomainError>;
}
