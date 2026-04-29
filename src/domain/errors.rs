use derive_more::{Display, From};
pub type Result<T> = core::result::Result<T, DomainError>;

#[derive(Debug, Display, From, PartialEq)]
#[display("Domain error: {_variant}")]
pub enum DomainError {
    InvalidAddress(String),
    InvalidContactInfo(String),
    InvalidBillingInfo(String),
    #[display("Invalid email: {_0}")]
    InvalidEmail(String),

    #[from(String,&str, &String)]
    ValidationError(String),

    #[display("Infrastructure error: {_0}")]
    InfrastructureError(String),
}

// region:    --- Error Boilerplate

// impl core::fmt::Display for DomainError {
//     fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
//         write!(fmt, "{self:?}")
//     }
// }

impl std::error::Error for DomainError {}

// endregion: --- Error Boilerplate
