/// Enum representing program types in the domain.
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramType {
    BibleStudy,
    YouthGroup,
    MensMinistry,
    WomensMinistry,
    ShortTermEvent,
    Other,
}

impl ProgramType {
    /// Returns a human-readable string representation of the program type.
    pub fn as_str(&self) -> &'static str {
        match self {
            ProgramType::BibleStudy => "BIBLE_STUDY",
            ProgramType::YouthGroup => "YOUTH_GROUP",
            ProgramType::MensMinistry => "MENS_MINISTRY",
            ProgramType::WomensMinistry => "WOMENS_MINISTRY",
            ProgramType::ShortTermEvent => "SHORT_TERM_EVENT",
            ProgramType::Other => "OTHER",
        }
    }

    /// Attempts to create a ProgramType from a string representation.
    /// Case-insensitive matching.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "bible_study" => Some(ProgramType::BibleStudy),
            "youth_group" => Some(ProgramType::YouthGroup),
            "mens_ministry" => Some(ProgramType::MensMinistry),
            "womens_ministry" => Some(ProgramType::WomensMinistry),
            "short_term_event" => Some(ProgramType::ShortTermEvent),
            "other" => Some(ProgramType::Other),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_type_variants() {
        assert_eq!(ProgramType::BibleStudy, ProgramType::BibleStudy);
        assert_eq!(ProgramType::YouthGroup, ProgramType::YouthGroup);
        assert_eq!(ProgramType::MensMinistry, ProgramType::MensMinistry);
        assert_eq!(ProgramType::WomensMinistry, ProgramType::WomensMinistry);
        assert_eq!(ProgramType::ShortTermEvent, ProgramType::ShortTermEvent);
        assert_eq!(ProgramType::Other, ProgramType::Other);

        assert_ne!(ProgramType::BibleStudy, ProgramType::YouthGroup);
    }

    #[test]
    fn test_program_type_as_str() {
        assert_eq!(ProgramType::BibleStudy.as_str(), "BIBLE_STUDY");
        assert_eq!(ProgramType::YouthGroup.as_str(), "YOUTH_GROUP");
        assert_eq!(ProgramType::MensMinistry.as_str(), "MENS_MINISTRY");
        assert_eq!(ProgramType::WomensMinistry.as_str(), "WOMENS_MINISTRY");
        assert_eq!(ProgramType::ShortTermEvent.as_str(), "SHORT_TERM_EVENT");
        assert_eq!(ProgramType::Other.as_str(), "OTHER");
    }

    #[test]
    fn test_program_type_from_str() {
        assert_eq!(
            ProgramType::from_str("bible_study"),
            Some(ProgramType::BibleStudy)
        );
        assert_eq!(
            ProgramType::from_str("YOUTH_GROUP"),
            Some(ProgramType::YouthGroup)
        );
        assert_eq!(
            ProgramType::from_str("mens_ministry"),
            Some(ProgramType::MensMinistry)
        );
        assert_eq!(
            ProgramType::from_str("WOMENS_MINISTRY"),
            Some(ProgramType::WomensMinistry)
        );
        assert_eq!(
            ProgramType::from_str("short_term_event"),
            Some(ProgramType::ShortTermEvent)
        );
        assert_eq!(ProgramType::from_str("OTHER"), Some(ProgramType::Other));
        assert_eq!(ProgramType::from_str("invalid"), None);
    }
}
