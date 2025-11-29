use std::fmt;

use masterror::AppError;

/// Component-level errors
#[derive(Debug, Clone)]
pub enum ComponentError {
    /// Failed to parse attribute
    AttributeParseError {
        /// Attribute name
        attribute: String,
        /// Invalid value
        value: String,
    },
    /// Failed to fetch markers from URL
    MarkerUrlFetchFailed {
        /// Marker URL
        url: String,
        /// HTTP status code
        status: u16,
    },
    /// Component not mounted
    ComponentNotMounted,
    /// Rendering error
    RenderError {
        /// Error details
        details: String,
    },
}

impl fmt::Display for ComponentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AttributeParseError { attribute, value } => {
                write!(f, "Failed to parse attribute '{attribute}': {value}")
            },
            Self::MarkerUrlFetchFailed { url, status } => {
                write!(f, "Failed to fetch markers from {url}: HTTP {status}")
            },
            Self::ComponentNotMounted => write!(f, "Component not mounted"),
            Self::RenderError { details } => write!(f, "Render error: {details}"),
        }
    }
}

impl std::error::Error for ComponentError {}

impl From<ComponentError> for AppError {
    fn from(err: ComponentError) -> Self {
        Self::internal(err.to_string())
    }
}
