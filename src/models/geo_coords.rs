//! Geographic coordinate structures.
//!
//! This module defines the coordinate representation used for property locations.

use serde::{Serialize, Deserialize};

/// Geographic coordinates (WGS84).
///
/// Represents a location on Earth using latitude and longitude in decimal degrees.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeoCoords {
    /// Latitude in decimal degrees (-90 to +90)
    pub latitude: f64,
    /// Longitude in decimal degrees (-180 to +180)
    pub longitude: f64
}