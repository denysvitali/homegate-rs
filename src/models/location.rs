//! Location metadata structures.
//!
//! This module defines location information structures used by the Homegate API.

use serde::{Serialize,Deserialize};

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
    #[serde(rename="typeLabel")]
    type_label: String
}