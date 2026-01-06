//! Real estate listing container and offer types.
//!
//! This module defines the top-level real estate structure and various
//! enumeration types for offer types, pricing units, and listing tiers.

use serde::{Deserialize, Serialize};

use crate::models::listing::Listing;

/// Real estate listing container.
///
/// This is the top-level structure returned in search results, wrapping
/// the detailed listing information.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RealEstate {
    /// Unique identifier for this real estate entry
    pub id: String,
    /// Detailed listing information
    pub listing: Listing,
}

/// Wrapper for listing type information.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListingTypeWrapper {
    /// The listing tier/type
    #[serde(rename = "type")]
    pub t: ListingType,
}

/// Type of real estate offer.
///
/// Indicates whether the property is for rent, sale, or other purpose.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum OfferType {
    /// Property is available for rent
    RENT,
}

/// Price unit enumeration.
///
/// Specifies the unit used for pricing calculations.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PriceUnit {
    /// Price per month
    MONTHLY,
    /// Price per square meter per year
    M2YEARLY,
    /// Price per week
    WEEKLY,
}

/// Listing tier/priority level.
///
/// Homegate uses different listing tiers that affect visibility and placement
/// in search results.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ListingType {
    /// Premium tier listing (highest visibility)
    PREMIUM,
    /// Top tier listing (high visibility)
    TOP,
    /// Standard tier listing (normal visibility)
    STANDARD,
}
