use std::fmt;
use std::io;

#[derive(Debug)]
#[allow(dead_code)] // Some variants are reserved for future use
pub enum ReconstructorError {
    IoError(io::Error),
    UnsupportedFormat,
    DwarfParseError(String),
    ObjectParseError(object::Error),
    GimliError(gimli::Error),
    NoDwarfData,
    ArchiveMemberError(String),
}

impl fmt::Display for ReconstructorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReconstructorError::IoError(err) => write!(f, "Failed to read file: {}", err),
            ReconstructorError::UnsupportedFormat => write!(f, "Unsupported file format"),
            ReconstructorError::DwarfParseError(msg) => {
                write!(f, "Failed to parse DWARF data: {}", msg)
            }
            ReconstructorError::ObjectParseError(err) => {
                write!(f, "Failed to parse object file: {}", err)
            }
            ReconstructorError::GimliError(err) => write!(f, "DWARF error: {}", err),
            ReconstructorError::NoDwarfData => write!(f, "No DWARF data found in file"),
            ReconstructorError::ArchiveMemberError(msg) => {
                write!(f, "Archive member parse error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ReconstructorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ReconstructorError::IoError(err) => Some(err),
            ReconstructorError::ObjectParseError(err) => Some(err),
            ReconstructorError::GimliError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for ReconstructorError {
    fn from(err: io::Error) -> Self {
        ReconstructorError::IoError(err)
    }
}

impl From<object::Error> for ReconstructorError {
    fn from(err: object::Error) -> Self {
        ReconstructorError::ObjectParseError(err)
    }
}

impl From<gimli::Error> for ReconstructorError {
    fn from(err: gimli::Error) -> Self {
        ReconstructorError::GimliError(err)
    }
}

pub type Result<T> = std::result::Result<T, ReconstructorError>;
