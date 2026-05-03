use crate::domain::{
    entities::Church,
    value_objects::{Address, ChurchId, ContactInfo, OrganizationId},
};
use crate::infrastructure::persistence::databases::postgres::errors::{PersistenceError, Result};

use super::dtos::ChurchDto;

/// Converts a database DTO to a domain Church entity.
pub fn dto_to_church(dto: ChurchDto) -> Result<Church> {
    let id = ChurchId::from(dto.id);
    let name = dto.name;

    let address: Address =
        serde_json::from_value(dto.address).map_err(PersistenceError::Serialization)?;

    let contact_info: ContactInfo =
        serde_json::from_value(dto.contact_info).map_err(PersistenceError::Serialization)?;

    let organization_id = OrganizationId::from(dto.organization_id);

    // Create the church manually since we have the raw data
    // Note: This bypasses validation, assuming database data is valid
    let church = Church::from_raw(
        id,
        name,
        address,
        contact_info,
        organization_id,
        dto.created_at,
        dto.updated_at,
    );

    Ok(church)
}

/// Converts a domain Church entity to a database DTO.
pub fn church_to_dto(church: &Church) -> Result<ChurchDto> {
    let id = church.id().value();
    let organization_id = church.organization_id().value();
    let name = church.name().to_string();

    let address =
        serde_json::to_value(church.address()).map_err(PersistenceError::Serialization)?;

    let contact_info =
        serde_json::to_value(church.contact_info()).map_err(PersistenceError::Serialization)?;

    let created_at = *church.created_at();
    let updated_at = *church.updated_at();

    Ok(ChurchDto {
        id,
        organization_id,
        pastor_user_id: None, // Not implemented in domain yet
        name,
        address,
        contact_info,
        created_at,
        updated_at,
    })
}
