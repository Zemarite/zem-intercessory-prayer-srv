use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{
    entities::Organization, errors::DomainError,
    repositories::organization_repository::OrganizationRepository, value_objects::OrganizationId,
};

use super::{dtos::OrganizationDto, mapping};

/// PostgreSQL implementation of the OrganizationRepository trait.
pub struct PostgresOrganizationRepository {
    pool: PgPool,
}

impl PostgresOrganizationRepository {
    /// Creates a new PostgreSQL organization repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrganizationRepository for PostgresOrganizationRepository {
    async fn create(&self, organization: Organization) -> Result<(), DomainError> {
        let dto = mapping::organization_to_dto(&organization)
            .map_err(|e| DomainError::InfrastructureError(format!("Mapping error: {}", e)))?;

        sqlx::query(
            r#"
            INSERT INTO organizations (id, name, contact_info, address, billing_info, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(&dto.id)
        .bind(&dto.name)
        .bind(&dto.contact_info)
        .bind(&dto.address)
        .bind(&dto.billing_info)
        .bind(&dto.created_at)
        .bind(&dto.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(format!("Database error: {}", e)))?;

        Ok(())
    }

    async fn update(&self, organization: &Organization) -> Result<(), DomainError> {
        let dto = mapping::organization_to_dto(organization)
            .map_err(|e| DomainError::InfrastructureError(format!("Mapping error: {}", e)))?;

        sqlx::query(
            r#"
            UPDATE organizations
            SET name = $2, contact_info = $3, address = $4, billing_info = $5, updated_at = $6
            WHERE id = $1
            "#,
        )
        .bind(&dto.id)
        .bind(&dto.name)
        .bind(&dto.contact_info)
        .bind(&dto.address)
        .bind(&dto.billing_info)
        .bind(&dto.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(format!("Database error: {}", e)))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &OrganizationId) -> Result<Option<Organization>, DomainError> {
        let dto = sqlx::query_as::<_, OrganizationDto>(
            r#"
            SELECT id, name, contact_info, address, billing_info, created_at, updated_at
            FROM organizations
            WHERE id = $1
            "#,
        )
        .bind(id.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(format!("Database error: {}", e)))?;

        match dto {
            Some(dto) => {
                let organization = mapping::dto_to_organization(dto).map_err(|e| {
                    DomainError::InfrastructureError(format!("Mapping error: {}", e))
                })?;
                Ok(Some(organization))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::enums::{ContactMethod, PaymentMethod};
    use crate::domain::value_objects::{Address, ContactInfo, Email};

    // Note: Integration tests require a PostgreSQL database.
    // To run these tests, set up a test database and use sqlx::test attribute.

    #[test]
    fn test_mapping_roundtrip() {
        use crate::domain::value_objects::BillingInfo;
        // Test that mapping from domain to DTO and back preserves data
        let address = Address::new(
            "123 Main St".to_string(),
            Some("Apt 4B".to_string()),
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        )
        .unwrap();

        let email = Email::new("contact@example.com".to_string()).unwrap();
        let contact_info =
            ContactInfo::new(email, "+1234567890".to_string(), ContactMethod::Email).unwrap();

        let billing_email = Email::new("billing@example.com".to_string()).unwrap();
        let billing = BillingInfo::new(
            PaymentMethod::CreditCard,
            billing_email,
            Some("TAX123".to_string()),
        )
        .unwrap();

        let organization = Organization::new(
            "Test Organization".to_string(),
            address.clone(),
            contact_info.clone(),
            Some(billing.clone()),
        )
        .unwrap();

        // Map to DTO
        let dto = mapping::organization_to_dto(&organization).unwrap();

        // Map back to domain
        let reconstructed = mapping::dto_to_organization(dto).unwrap();

        assert_eq!(organization.id(), reconstructed.id());
        assert_eq!(organization.name(), reconstructed.name());
        assert_eq!(organization.address(), reconstructed.address());
        assert_eq!(organization.contact_info(), reconstructed.contact_info());
        assert_eq!(organization.billing(), reconstructed.billing());
        assert_eq!(organization.created_at(), reconstructed.created_at());
        assert_eq!(organization.updated_at(), reconstructed.updated_at());
    }
}
