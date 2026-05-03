use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// Database transfer object for Church.
/// This struct represents the raw data from the database before mapping to domain entities.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChurchDto {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub pastor_user_id: Option<Uuid>, // Not used in domain yet
    pub name: String,
    pub address: serde_json::Value,
    pub contact_info: serde_json::Value,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
