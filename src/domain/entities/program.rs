use time::{Date, OffsetDateTime};

use crate::domain::enums::ProgramType;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{ChurchId, MemberId, ProgramId};

/// Represents a Program entity in the domain.
/// Entities have identity and are compared by their ID, not by value.
#[derive(Debug, Clone)]
pub struct Program {
    id: ProgramId,
    church_id: ChurchId,
    name: String,
    description: Option<String>,
    program_type: ProgramType,
    is_temporary: bool,
    start_date: Option<Date>,
    end_date: Option<Date>,
    leader_user_id: Option<MemberId>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl Program {
    /// Creates a new Program entity with a generated ID and current timestamps.
    pub fn new(
        church_id: ChurchId,
        name: String,
        description: Option<String>,
        program_type: ProgramType,
        is_temporary: bool,
        start_date: Option<Date>,
        end_date: Option<Date>,
        leader_user_id: Option<MemberId>,
    ) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Program name cannot be empty".to_string(),
            ));
        }
        if name.len() > 64 {
            return Err(DomainError::ValidationError(
                "Program name cannot exceed 64 characters".to_string(),
            ));
        }
        if let Some(desc) = &description {
            if desc.len() > 256 {
                return Err(DomainError::ValidationError(
                    "Program description cannot exceed 256 characters".to_string(),
                ));
            }
        }
        if let (Some(start), Some(end)) = (start_date, end_date) {
            if start > end {
                return Err(DomainError::ValidationError(
                    "Start date cannot be after end date".to_string(),
                ));
            }
        }

        let now = OffsetDateTime::now_utc();

        Ok(Self {
            id: ProgramId::generate(),
            church_id,
            name,
            description,
            program_type,
            is_temporary,
            start_date,
            end_date,
            leader_user_id,
            created_at: now,
            updated_at: now,
        })
    }

    // region: Getters for encapsulation
    pub fn id(&self) -> &ProgramId {
        &self.id
    }

    pub fn church_id(&self) -> &ChurchId {
        &self.church_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn program_type(&self) -> &ProgramType {
        &self.program_type
    }

    pub fn is_temporary(&self) -> bool {
        self.is_temporary
    }

    pub fn start_date(&self) -> &Option<Date> {
        &self.start_date
    }

    pub fn end_date(&self) -> &Option<Date> {
        &self.end_date
    }

    pub fn leader_user_id(&self) -> &Option<MemberId> {
        &self.leader_user_id
    }

    pub fn created_at(&self) -> &OffsetDateTime {
        &self.created_at
    }

    pub fn updated_at(&self) -> &OffsetDateTime {
        &self.updated_at
    }
    // endregion:Getters for encapsulation

    // Business behavior / methods
    /// Updates the program's name and sets the updated_at timestamp.
    pub fn update_name(&mut self, new_name: String) -> Result<(), DomainError> {
        if new_name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Program name cannot be empty".to_string(),
            ));
        }
        if new_name.len() > 64 {
            return Err(DomainError::ValidationError(
                "Program name cannot exceed 64 characters".to_string(),
            ));
        }

        self.name = new_name;
        self.updated_at = OffsetDateTime::now_utc();
        Ok(())
    }

    /// Updates the program's description and sets the updated_at timestamp.
    pub fn update_description(
        &mut self,
        new_description: Option<String>,
    ) -> Result<(), DomainError> {
        if let Some(desc) = &new_description {
            if desc.len() > 256 {
                return Err(DomainError::ValidationError(
                    "Program description cannot exceed 256 characters".to_string(),
                ));
            }
        }

        self.description = new_description;
        self.updated_at = OffsetDateTime::now_utc();
        Ok(())
    }

    /// Updates the program's type and sets the updated_at timestamp.
    pub fn update_program_type(&mut self, new_program_type: ProgramType) {
        self.program_type = new_program_type;
        self.updated_at = OffsetDateTime::now_utc();
    }

    /// Updates the program's temporary status and sets the updated_at timestamp.
    pub fn update_is_temporary(&mut self, new_is_temporary: bool) {
        self.is_temporary = new_is_temporary;
        self.updated_at = OffsetDateTime::now_utc();
    }

    /// Updates the program's dates and sets the updated_at timestamp.
    pub fn update_dates(
        &mut self,
        new_start_date: Option<Date>,
        new_end_date: Option<Date>,
    ) -> Result<(), DomainError> {
        if let (Some(start), Some(end)) = (new_start_date, new_end_date) {
            if start > end {
                return Err(DomainError::ValidationError(
                    "Start date cannot be after end date".to_string(),
                ));
            }
        }

        self.start_date = new_start_date;
        self.end_date = new_end_date;
        self.updated_at = OffsetDateTime::now_utc();
        Ok(())
    }

    /// Updates the program's leader and sets the updated_at timestamp.
    pub fn update_leader(&mut self, new_leader_user_id: Option<MemberId>) {
        self.leader_user_id = new_leader_user_id;
        self.updated_at = OffsetDateTime::now_utc();
    }
}

impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Program {}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[test]
    fn test_program_creation() {
        let church_id = ChurchId::generate();
        let leader_id = Some(MemberId::generate());
        let start_date = Some(date!(2024 - 01 - 01));
        let end_date = Some(date!(2024 - 12 - 31));

        let program = Program::new(
            church_id.clone(),
            "Bible Study Group".to_string(),
            Some("Weekly Bible study".to_string()),
            ProgramType::BibleStudy,
            false,
            start_date,
            end_date,
            leader_id.clone(),
        )
        .expect("Program creation should succeed");

        assert_eq!(program.church_id(), &church_id);
        assert_eq!(program.name(), "Bible Study Group");
        assert_eq!(
            program.description().as_ref().unwrap(),
            "Weekly Bible study"
        );
        assert_eq!(program.program_type(), &ProgramType::BibleStudy);
        assert!(!program.is_temporary());
        assert_eq!(program.start_date(), &start_date);
        assert_eq!(program.end_date(), &end_date);
        assert_eq!(program.leader_user_id(), &leader_id);
        assert!(program.created_at() <= &OffsetDateTime::now_utc());
        assert!(program.updated_at() <= &OffsetDateTime::now_utc());
    }

    #[test]
    fn test_program_creation_fails_with_empty_name() {
        let church_id = ChurchId::generate();

        let result = Program::new(
            church_id,
            "".to_string(),
            None,
            ProgramType::Other,
            false,
            None,
            None,
            None,
        );

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::ValidationError("Program name cannot be empty".to_string())
        );
    }

    #[test]
    fn test_program_creation_fails_with_long_name() {
        let church_id = ChurchId::generate();
        let long_name = "a".repeat(65);

        let result = Program::new(
            church_id,
            long_name,
            None,
            ProgramType::Other,
            false,
            None,
            None,
            None,
        );

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::ValidationError("Program name cannot exceed 64 characters".to_string())
        );
    }

    #[test]
    fn test_program_creation_fails_with_long_description() {
        let church_id = ChurchId::generate();
        let long_desc = "a".repeat(257);

        let result = Program::new(
            church_id,
            "Test Program".to_string(),
            Some(long_desc),
            ProgramType::Other,
            false,
            None,
            None,
            None,
        );

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::ValidationError(
                "Program description cannot exceed 256 characters".to_string()
            )
        );
    }

    #[test]
    fn test_program_creation_fails_with_invalid_dates() {
        let church_id = ChurchId::generate();
        let start_date = Some(date!(2024 - 12 - 31));
        let end_date = Some(date!(2024 - 01 - 01));

        let result = Program::new(
            church_id,
            "Test Program".to_string(),
            None,
            ProgramType::Other,
            false,
            start_date,
            end_date,
            None,
        );

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::ValidationError("Start date cannot be after end date".to_string())
        );
    }

    #[test]
    fn test_program_update_name() {
        let mut program = create_test_program();
        let new_name = "Updated Bible Study";

        program.update_name(new_name.to_string()).unwrap();

        assert_eq!(program.name(), new_name);
        assert!(program.updated_at() > program.created_at());
    }

    #[test]
    fn test_program_update_description() {
        let mut program = create_test_program();
        let new_desc = Some("Updated description".to_string());

        program.update_description(new_desc.clone()).unwrap();

        assert_eq!(program.description(), &new_desc);
        assert!(program.updated_at() > program.created_at());
    }

    #[test]
    fn test_program_update_program_type() {
        let mut program = create_test_program();
        let new_type = ProgramType::YouthGroup;

        program.update_program_type(new_type.clone());

        assert_eq!(program.program_type(), &new_type);
        assert!(program.updated_at() > program.created_at());
    }

    #[test]
    fn test_program_update_is_temporary() {
        let mut program = create_test_program();

        program.update_is_temporary(true);

        assert!(program.is_temporary());
        assert!(program.updated_at() > program.created_at());
    }

    #[test]
    fn test_program_update_dates() {
        let mut program = create_test_program();
        let new_start = Some(date!(2024 - 06 - 01));
        let new_end = Some(date!(2024 - 06 - 30));

        program.update_dates(new_start, new_end).unwrap();

        assert_eq!(program.start_date(), &new_start);
        assert_eq!(program.end_date(), &new_end);
        assert!(program.updated_at() > program.created_at());
    }

    #[test]
    fn test_program_update_leader() {
        let mut program = create_test_program();
        let new_leader = Some(MemberId::generate());

        program.update_leader(new_leader.clone());

        assert_eq!(program.leader_user_id(), &new_leader);
        assert!(program.updated_at() > program.created_at());
    }

    fn create_test_program() -> Program {
        let church_id = ChurchId::generate();
        Program::new(
            church_id,
            "Test Program".to_string(),
            Some("Test description".to_string()),
            ProgramType::BibleStudy,
            false,
            Some(date!(2024 - 01 - 01)),
            Some(date!(2024 - 12 - 31)),
            Some(MemberId::generate()),
        )
        .unwrap()
    }
}
