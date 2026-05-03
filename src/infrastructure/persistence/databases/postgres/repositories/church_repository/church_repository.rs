use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{
    entities::Church,
    errors::DomainError,
    repositories::church_repository::ChurchRepository,
    value_objects::{ChurchId, OrganizationId},
};

use super::{dtos::ChurchDto, mapping};

/// PostgreSQL implementation of the ChurchRepository trait.
pub struct PostgresChurchRepository {
    pool: PgPool,
}

impl PostgresChurchRepository {
    /// Creates a new PostgreSQL church repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChurchRepository for PostgresChurchRepository {
    async fn create(&self, church: Church) -> Result<(), DomainError> {
        let dto = mapping::church_to_dto(&church)
            .map_err(|e| DomainError::InfrastructureError(format!("Mapping error: {}", e)))?;

        sqlx::query(
            r#"
            INSERT INTO church (id, organization_id, pastor_user_id, name, address, contact_info, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(&dto.id)
        .bind(&dto.organization_id)
        .bind(&dto.pastor_user_id)
        .bind(&dto.name)
        .bind(&dto.address)
        .bind(&dto.contact_info)
        .bind(&dto.created_at)
        .bind(&dto.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(format!("Database error: {}", e)))?;

        Ok(())
    }

    async fn update(&self, church: &Church) -> Result<(), DomainError> {
        let dto = mapping::church_to_dto(church)
            .map_err(|e| DomainError::InfrastructureError(format!("Mapping error: {}", e)))?;

        sqlx::query(
            r#"
            UPDATE church
            SET name = $2, address = $3, contact_info = $4, updated_at = $5
            WHERE id = $1
            "#,
        )
        .bind(&dto.id)
        .bind(&dto.name)
        .bind(&dto.address)
        .bind(&dto.contact_info)
        .bind(&dto.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(format!("Database error: {}", e)))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &ChurchId) -> Result<Option<Church>, DomainError> {
        let dto = sqlx::query_as::<_, ChurchDto>(
            r#"
            SELECT id, organization_id, pastor_user_id, name, address, contact_info, created_at, updated_at
            FROM church
            WHERE id = $1
            "#,
        )
        .bind(id.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(format!("Database error: {}", e)))?;

        match dto {
            Some(dto) => {
                let church = mapping::dto_to_church(dto).map_err(|e| {
                    DomainError::InfrastructureError(format!("Mapping error: {}", e))
                })?;
                Ok(Some(church))
            }
            None => Ok(None),
        }
    }

    async fn list_by_organization_id(
        &self,
        organization_id: &OrganizationId,
    ) -> Result<Vec<Church>, DomainError> {
        let dtos = sqlx::query_as::<_, ChurchDto>(
            r#"
            SELECT id, organization_id, pastor_user_id, name, address, contact_info, created_at, updated_at
            FROM church
            WHERE organization_id = $1
            "#,
        )
        .bind(organization_id.value())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(format!("Database error: {}", e)))?;

        let mut churches = Vec::new();
        for dto in dtos {
            let church = mapping::dto_to_church(dto)
                .map_err(|e| DomainError::InfrastructureError(format!("Mapping error: {}", e)))?;
            churches.push(church);
        }

        Ok(churches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
