use serde::{Deserialize, Serialize};

use crate::{error::CoreError, projection::GeoCoord};

/// Unique identifier for a marker
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkerId {
    /// String identifier
    String(String),
    /// Numeric identifier
    Number(u64),
}

impl MarkerId {
    /// Validates the marker ID
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidMarkerId`] if the string ID is empty
    pub fn validate(&self) -> Result<(), CoreError> {
        match self {
            Self::String(s) if s.is_empty() => Err(CoreError::InvalidMarkerId { id: s.clone() }),
            _ => Ok(()),
        }
    }
}

/// A map marker with position, appearance, and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker {
    /// Unique marker identifier
    pub id: MarkerId,
    /// Geographic coordinates
    pub coord: GeoCoord,
    /// Glow intensity (0.0 to 1.0)
    #[serde(default = "default_intensity")]
    pub intensity: f32,
    /// Optional custom color
    #[serde(default)]
    pub color: Option<Color>,
    /// Marker radius in pixels
    #[serde(default = "default_radius")]
    pub radius: f32,
    /// Optional metadata
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
}

const fn default_intensity() -> f32 {
    1.0
}

const fn default_radius() -> f32 {
    5.0
}

impl Marker {
    /// Creates a new marker with the specified ID and coordinates
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidMarkerId`] if the ID is invalid or
    /// [`CoreError::InvalidCoordinates`] if the coordinates are out of bounds
    pub fn new(id: MarkerId, lat: f64, lon: f64) -> Result<Self, CoreError> {
        id.validate()?;
        let coord = GeoCoord::new(lat, lon)?;

        Ok(Self {
            id,
            coord,
            intensity: default_intensity(),
            color: None,
            radius: default_radius(),
            meta: None,
        })
    }

    /// Validates the marker data
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidMarkerId`] if intensity or radius are invalid
    pub fn validate(&self) -> Result<(), CoreError> {
        self.id.validate()?;

        if !(0.0..=1.0).contains(&self.intensity) {
            return Err(CoreError::InvalidMarkerId {
                id: format!("intensity {} out of range [0, 1]", self.intensity),
            });
        }

        if self.radius <= 0.0 || !self.radius.is_finite() {
            return Err(CoreError::InvalidMarkerId {
                id: format!("invalid radius {}", self.radius),
            });
        }

        Ok(())
    }

    /// Computes a deterministic animation phase based on marker ID
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn phase(&self) -> f32 {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };

        let mut hasher = DefaultHasher::new();
        self.id.hash(&mut hasher);
        let hash = hasher.finish();
        let normalized = (hash % 10000) as f32 / 10000.0;
        normalized * std::f32::consts::TAU
    }
}

/// RGBA color with components in range [0.0, 1.0]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    /// Red component
    pub r: f32,
    /// Green component
    pub g: f32,
    /// Blue component
    pub b: f32,
    /// Alpha component
    pub a: f32,
}

impl Color {
    /// Creates a color from RGBA components
    #[must_use]
    #[allow(clippy::many_single_char_names)]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Parses a color from a hex string (e.g., "#FF0000" or "#FF0000FF")
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::ThemeValidationFailed`] if the hex string is invalid
    pub fn from_hex(hex: &str) -> Result<Self, CoreError> {
        let hex = hex.trim_start_matches('#');

        if hex.len() != 6 && hex.len() != 8 {
            return Err(CoreError::ThemeValidationFailed {
                reason: format!("invalid hex color: {hex}"),
            });
        }

        let r =
            u8::from_str_radix(&hex[0..2], 16).map_err(|_| CoreError::ThemeValidationFailed {
                reason: format!("invalid hex color: {hex}"),
            })?;
        let g =
            u8::from_str_radix(&hex[2..4], 16).map_err(|_| CoreError::ThemeValidationFailed {
                reason: format!("invalid hex color: {hex}"),
            })?;
        let b =
            u8::from_str_radix(&hex[4..6], 16).map_err(|_| CoreError::ThemeValidationFailed {
                reason: format!("invalid hex color: {hex}"),
            })?;

        let a = if hex.len() == 8 {
            u8::from_str_radix(&hex[6..8], 16).map_err(|_| CoreError::ThemeValidationFailed {
                reason: format!("invalid hex color: {hex}"),
            })?
        } else {
            255
        };

        Ok(Self {
            r: f32::from(r) / 255.0,
            g: f32::from(g) / 255.0,
            b: f32::from(b) / 255.0,
            a: f32::from(a) / 255.0,
        })
    }
}
