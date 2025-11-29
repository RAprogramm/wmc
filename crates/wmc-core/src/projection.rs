use serde::{Deserialize, Serialize};

use crate::error::CoreError;

/// Geographic coordinate in WGS84 (latitude, longitude)
///
/// # Examples
///
/// ```
/// use wmc_core::projection::GeoCoord;
///
/// let moscow = GeoCoord::new(55.7558, 37.6173).unwrap();
/// assert_eq!(moscow.lat, 55.7558);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GeoCoord {
    /// Latitude in degrees (-90 to 90)
    pub lat: f64,
    /// Longitude in degrees (-180 to 180)
    pub lon: f64,
}

impl GeoCoord {
    /// Creates new geographic coordinate with validation
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidCoordinates`] if:
    /// - Latitude not in range [-90, 90]
    /// - Longitude not in range [-180, 180]
    /// - Values are NaN or Infinity
    ///
    /// # Examples
    ///
    /// ```
    /// use wmc_core::projection::GeoCoord;
    ///
    /// let valid = GeoCoord::new(0.0, 0.0);
    /// assert!(valid.is_ok());
    ///
    /// let invalid = GeoCoord::new(91.0, 0.0);
    /// assert!(invalid.is_err());
    /// ```
    pub fn new(lat: f64, lon: f64) -> Result<Self, CoreError> {
        if !lat.is_finite() || !lon.is_finite() {
            return Err(CoreError::InvalidCoordinates { lat, lon });
        }

        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
            return Err(CoreError::InvalidCoordinates { lat, lon });
        }

        Ok(Self { lat, lon })
    }
}

/// Projected coordinate in screen space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProjectedCoord {
    /// X coordinate in pixels
    pub x: f64,
    /// Y coordinate in pixels
    pub y: f64,
}

/// Map projection trait for converting between geographic and screen coordinates
pub trait Projection {
    /// Projects geographic coordinates to screen space
    fn project(&self, coord: GeoCoord) -> ProjectedCoord;
    /// Unprojects screen coordinates to geographic space
    fn unproject(&self, coord: ProjectedCoord) -> GeoCoord;
}

/// Web Mercator projection
#[derive(Debug, Clone, Copy)]
pub struct MercatorProjection {
    width: f64,
    height: f64,
}

impl MercatorProjection {
    /// Creates a new Mercator projection with the specified dimensions
    #[must_use]
    pub const fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Projection for MercatorProjection {
    fn project(&self, coord: GeoCoord) -> ProjectedCoord {
        let lon_rad = coord.lon.to_radians();
        let lat_rad = coord.lat.to_radians();

        let x = (lon_rad + std::f64::consts::PI) / (2.0 * std::f64::consts::PI) * self.width;
        let y = (1.0 - (lat_rad.tan() + (1.0 / lat_rad.cos())).ln() / std::f64::consts::PI) / 2.0
            * self.height;

        ProjectedCoord { x, y }
    }

    fn unproject(&self, coord: ProjectedCoord) -> GeoCoord {
        let lon =
            (coord.x / self.width * 2.0).mul_add(std::f64::consts::PI, -std::f64::consts::PI);
        let lat_rad = ((std::f64::consts::PI
            - 2.0 * std::f64::consts::PI * coord.y / self.height)
            .exp()
            .atan())
        .mul_add(2.0, -(std::f64::consts::PI / 2.0));

        GeoCoord {
            lat: lat_rad.to_degrees(),
            lon: lon.to_degrees(),
        }
    }
}

/// Equirectangular (Plate Carrée) projection
#[derive(Debug, Clone, Copy)]
pub struct EquirectangularProjection {
    width: f64,
    height: f64,
}

impl EquirectangularProjection {
    /// Creates a new equirectangular projection with the specified dimensions
    #[must_use]
    pub const fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Projection for EquirectangularProjection {
    fn project(&self, coord: GeoCoord) -> ProjectedCoord {
        let x = (coord.lon + 180.0) / 360.0 * self.width;
        let y = (90.0 - coord.lat) / 180.0 * self.height;

        ProjectedCoord { x, y }
    }

    fn unproject(&self, coord: ProjectedCoord) -> GeoCoord {
        let lon = (coord.x / self.width).mul_add(360.0, -180.0);
        let lat = (coord.y / self.height).mul_add(-180.0, 90.0);

        GeoCoord { lat, lon }
    }
}
