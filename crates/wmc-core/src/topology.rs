use crate::{error::CoreError, projection::GeoCoord};

/// World map topology data
#[derive(Debug, Clone)]
pub struct WorldTopology {
    /// Vector of geographic features
    pub features: Vec<Feature>,
}

/// A geographic feature with geometry
#[derive(Debug, Clone)]
pub struct Feature {
    /// Feature geometry
    pub geometry: Geometry,
}

/// Geometric representation of geographic features
#[derive(Debug, Clone)]
pub enum Geometry {
    /// Single line string
    LineString(Vec<GeoCoord>),
    /// Multiple line strings
    MultiLineString(Vec<Vec<GeoCoord>>),
}

impl WorldTopology {
    /// Parses world topology from `GeoJSON` string
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::TopologyParseError`] if the `GeoJSON` is invalid
    pub fn from_geojson(geojson_str: &str) -> Result<Self, CoreError> {
        let geojson = geojson_str.parse::<geojson::GeoJson>().map_err(|e| {
            CoreError::TopologyParseError {
                details: e.to_string(),
            }
        })?;

        let features = match geojson {
            geojson::GeoJson::FeatureCollection(fc) => fc
                .features
                .into_iter()
                .filter_map(|f| f.geometry)
                .filter_map(|g| Self::parse_geometry(g).ok())
                .collect(),
            _ => {
                return Err(CoreError::TopologyParseError {
                    details: "Expected FeatureCollection".to_string(),
                });
            },
        };

        Ok(Self { features })
    }

    fn parse_geometry(geometry: geojson::Geometry) -> Result<Feature, CoreError> {
        let geom = match geometry.value {
            geojson::Value::LineString(coords) => {
                let points = coords
                    .into_iter()
                    .filter_map(|c| {
                        if c.len() >= 2 {
                            GeoCoord::new(c[1], c[0]).ok()
                        } else {
                            None
                        }
                    })
                    .collect();
                Geometry::LineString(points)
            },
            geojson::Value::MultiLineString(lines) => {
                let multi = lines
                    .into_iter()
                    .map(|line| {
                        line.into_iter()
                            .filter_map(|c| {
                                if c.len() >= 2 {
                                    GeoCoord::new(c[1], c[0]).ok()
                                } else {
                                    None
                                }
                            })
                            .collect()
                    })
                    .collect();
                Geometry::MultiLineString(multi)
            },
            geojson::Value::Polygon(rings) => {
                let multi = rings
                    .into_iter()
                    .map(|ring| {
                        ring.into_iter()
                            .filter_map(|c| {
                                if c.len() >= 2 {
                                    GeoCoord::new(c[1], c[0]).ok()
                                } else {
                                    None
                                }
                            })
                            .collect()
                    })
                    .collect();
                Geometry::MultiLineString(multi)
            },
            geojson::Value::MultiPolygon(polygons) => {
                let multi = polygons
                    .into_iter()
                    .flat_map(|poly| {
                        poly.into_iter().map(|ring| {
                            ring.into_iter()
                                .filter_map(|c| {
                                    if c.len() >= 2 {
                                        GeoCoord::new(c[1], c[0]).ok()
                                    } else {
                                        None
                                    }
                                })
                                .collect()
                        })
                    })
                    .collect();
                Geometry::MultiLineString(multi)
            },
            _ => {
                return Err(CoreError::TopologyParseError {
                    details: "Unsupported geometry type".to_string(),
                });
            },
        };

        Ok(Feature { geometry: geom })
    }

    /// Returns the total number of line strings in the topology
    #[must_use]
    pub fn line_count(&self) -> usize {
        self.features
            .iter()
            .map(|f| match &f.geometry {
                Geometry::LineString(_) => 1,
                Geometry::MultiLineString(lines) => lines.len(),
            })
            .sum()
    }

    /// Returns the total number of coordinate points in the topology
    #[must_use]
    pub fn point_count(&self) -> usize {
        self.features
            .iter()
            .map(|f| match &f.geometry {
                Geometry::LineString(points) => points.len(),
                Geometry::MultiLineString(lines) => lines.iter().map(std::vec::Vec::len).sum(),
            })
            .sum()
    }
}
