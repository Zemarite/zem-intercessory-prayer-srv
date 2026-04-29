use crate::domain::entities::Organization;
use crate::domain::value_objects::{Address, BillingInfo, ContactInfo, OrganizationId};
use crate::infrastructure::persistence::databases::postgres::errors::{PersistenceError, Result};

use super::dtos::OrganizationDto;

/// Converts a database DTO to a domain Organization entity.
pub fn dto_to_organization(dto: OrganizationDto) -> Result<Organization> {
    let id = OrganizationId::from(dto.id);
    let name = dto.name;

    let address: Address = serde_json::from_value(dto.address).map_err(PersistenceError::Serialization)?;

    let contact_info: ContactInfo =
        serde_json::from_value(dto.contact_info).map_err(PersistenceError::Serialization)?;

    let billing: Option<BillingInfo> = if let Some(billing_json) = dto.billing_info {
        Some(serde_json::from_value(billing_json).map_err(PersistenceError::Serialization)?)
    } else {
        None
    };

    // Create the organization manually since we have the raw data
    // Note: This bypasses validation, assuming database data is valid
    let organization = Organization::from_raw(
        id,
        name,
        address,
        contact_info,
        billing,
        dto.created_at,
        dto.updated_at,
    );

    Ok(organization)
}

/// Converts a domain Organization entity to a database DTO.
pub fn organization_to_dto(organization: &Organization) -> Result<OrganizationDto> {
    let id = organization.id().value();
    let name = organization.name().to_string();

    let address = serde_json::to_value(organization.address()).map_err(PersistenceError::Serialization)?;

    let contact_info =
        serde_json::to_value(organization.contact_info()).map_err(PersistenceError::Serialization)?;

    let billing_info = if let Some(billing) = organization.billing() {
        Some(serde_json::to_value(billing).map_err(PersistenceError::Serialization)?)
    } else {
        None
    };

    let created_at = *organization.created_at();
    let updated_at = *organization.updated_at();

    Ok(OrganizationDto {
        id,
        name,
        contact_info,
        address,
        billing_info,
        created_at,
        updated_at,
    })
}
