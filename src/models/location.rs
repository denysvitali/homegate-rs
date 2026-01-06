//! Location metadata structures.
//!
//! This module defines location information structures used by the Homegate API.

use serde::{Deserialize, Serialize};

/// Location metadata.
///
/// Represents a named location with type information (e.g., city, region).
#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    /// Location name (e.g., "Zürich")
    name: String,
    /// Location type identifier
    r#type: String,
    /// Human-readable location type label
    #[serde(rename = "typeLabel")]
    type_label: String,
}

impl Location {
    /// Returns the location name (e.g., "Zürich")
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the location type identifier
    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    /// Returns the human-readable location type label
    pub fn type_label(&self) -> &str {
        &self.type_label
    }
}
