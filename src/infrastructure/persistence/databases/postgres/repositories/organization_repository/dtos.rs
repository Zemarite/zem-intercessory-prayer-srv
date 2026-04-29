use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// Database transfer object for Organization.
/// This struct represents the raw data from the database before mapping to domain entities.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrganizationDto {
    pub id: Uuid,
    pub name: String,
    pub contact_info: serde_json::Value,
    pub address: serde_json::Value,
    pub billing_info: Option<serde_json::Value>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}