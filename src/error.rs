use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)] // Some variants are reserved for future use
pub enum ReconstructorError {
    #[error("Failed to read file: {0}")]
    IoError(#[from] io::Error),

    #[error("Unsupported file format")]
    UnsupportedFormat,

    #[error("Failed to parse DWARF data: {0}")]
    DwarfParseError(String),

    #[error("Failed to parse object file: {0}")]
    ObjectParseError(#[from] object::Error),

    #[error("DWARF error: {0}")]
    GimliError(#[from] gimli::Error),

    #[error("No DWARF data found in file")]
    NoDwarfData,

    #[error("Archive member parse error: {0}")]
    ArchiveMemberError(String),
}

pub type Result<T> = std::result::Result<T, ReconstructorError>;

// Helper to convert Box<dyn Error> from legacy code to our error type
impl From<Box<dyn std::error::Error>> for ReconstructorError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        ReconstructorError::DwarfParseError(err.to_string())
    }
}
