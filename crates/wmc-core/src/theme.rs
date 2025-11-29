use serde::{Deserialize, Serialize};

use crate::{error::CoreError, marker::Color};

/// Visual theme for map rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Background color
    pub background: Color,
    /// Contour line color
    pub contour_color: Color,
    /// Contour line width in pixels
    pub contour_width: f32,
    /// Default marker color
    pub marker_color: Color,
    /// Marker glow radius
    pub marker_glow: f32,
    /// Film grain intensity
    pub grain_intensity: f32,
}

impl Theme {
    /// Creates a dark minimal theme preset
    #[must_use]
    pub fn dark_minimal() -> Self {
        Self {
            name: "dark-minimal".to_string(),
            background: Color::rgba(0.043, 0.059, 0.063, 1.0), // #0b0f10
            contour_color: Color::rgba(0.12, 0.15, 0.16, 0.6),
            contour_width: 1.0,
            marker_color: Color::rgba(0.3, 0.7, 0.8, 1.0),
            marker_glow: 8.0,
            grain_intensity: 0.015,
        }
    }

    /// Validates the theme data
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::ThemeValidationFailed`] if any theme values are invalid
    pub fn validate(&self) -> Result<(), CoreError> {
        if self.name.is_empty() {
            return Err(CoreError::ThemeValidationFailed {
                reason: "empty theme name".to_string(),
            });
        }

        if self.contour_width <= 0.0 || !self.contour_width.is_finite() {
            return Err(CoreError::ThemeValidationFailed {
                reason: format!("invalid contour width: {}", self.contour_width),
            });
        }

        if self.marker_glow < 0.0 || !self.marker_glow.is_finite() {
            return Err(CoreError::ThemeValidationFailed {
                reason: format!("invalid marker glow: {}", self.marker_glow),
            });
        }

        Ok(())
    }
}
