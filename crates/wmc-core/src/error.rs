use std::fmt;

use masterror::AppError;

/// Core library errors
#[derive(Debug, Clone)]
pub enum CoreError {
    /// Invalid geographic coordinates
    InvalidCoordinates {
        /// Latitude value
        lat: f64,
        /// Longitude value
        lon: f64,
    },
    /// Invalid marker identifier
    InvalidMarkerId {
        /// Marker ID or error description
        id: String,
    },
    /// Theme validation failed
    ThemeValidationFailed {
        /// Validation failure reason
        reason: String,
    },
    /// Topology parsing error
    TopologyParseError {
        /// Error details
        details: String,
    },
    /// Buffer overflow error
    BufferOverflow {
        /// Requested size
        requested: usize,
        /// Available capacity
        capacity: usize,
    },
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCoordinates { lat, lon } => {
                write!(f, "Invalid coordinates: lat={lat}, lon={lon}")
            },
            Self::InvalidMarkerId { id } => write!(f, "Invalid marker ID: {id}"),
            Self::ThemeValidationFailed { reason } => {
                write!(f, "Theme validation failed: {reason}")
            },
            Self::TopologyParseError { details } => {
                write!(f, "Topology parse error: {details}")
            },
            Self::BufferOverflow {
                requested,
                capacity,
            } => {
                write!(
                    f,
                    "Buffer overflow: requested {requested} but capacity is {capacity}"
                )
            },
        }
    }
}

impl std::error::Error for CoreError {}

impl From<CoreError> for AppError {
    fn from(err: CoreError) -> Self {
        Self::internal(err.to_string())
    }
}
