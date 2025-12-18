//! # Homegate - Unofficial Rust Library
//!
//! `homegate` is an unofficial Rust library for interacting with the [Homegate.ch](https://homegate.ch)
//! real estate API. This library provides a simple interface to search for rental and sale properties
//! in Switzerland.
//!
//! ## Warning: Unofficial API Usage
//!
//! **This library uses unofficial APIs extracted from the Homegate mobile application.**
//!
//! - Your account may be banned for excessive API usage or scraping
//! - The API endpoints and authentication may change without notice
//! - All data accessed through this library remains the property of SMG Swiss Marketplace Group Ltd
//! - Commercial use or republication of this data is prohibited without explicit permission
//!
//! **This library is intended for educational purposes and personal use only.**
//!
//! ## Quick Start
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! homegate = "1.0"
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! ## Basic Usage
//!
//! Search for properties in a specific location:
//!
//! ```no_run
//! use homegate::api::search::{search, Location};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Search for properties in Zurich within 1km radius
//! let location = Location {
//!     latitude: 47.36667,
//!     longitude: 8.55,
//!     radius: 1000, // meters
//! };
//!
//! let results = search(&location).await?;
//! println!("Found {} total listings", results.total);
//!
//! // Iterate through results
//! for real_estate in results.results {
//!     let listing = real_estate.listing;
//!     println!("Property: {}", listing.id);
//!     println!("  Address: {:?}", listing.address.locality);
//!     println!("  Rooms: {}", listing.characteristics.number_of_rooms);
//!     println!("  Size: {}m²", listing.characteristics.living_space);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Features
//!
//! - **Property Search**: Search for real estate listings by location, price, size, and category
//! - **Rich Data Models**: Comprehensive data structures for listings, addresses, prices, and more
//! - **Type-Safe**: Strongly typed API with full serde support for JSON serialization
//! - **Async/Await**: Built on `tokio` and `reqwest` for async HTTP requests
//! - **Pagination**: Support for paginated search results
//!
//! ## Module Overview
//!
//! - [`api`] - API client functionality and search operations
//!   - [`api::search`] - Search for real estate listings
//! - [`models`] - Data structures for API responses
//!   - [`models::listing`] - Listing details and property categories
//!   - [`models::realestate`] - Real estate containers and offer types
//!   - [`models::address`] - Address and location data
//!   - [`models::paginated`] - Paginated response handling
//!
//! ## Advanced Example
//!
//! ```no_run
//! use homegate::api::search::{default_search, Location, FromTo, FromToFloat};
//! use homegate::api::request::post_url;
//! use homegate::api::BACKEND_URL;
//! use reqwest::Url;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a custom search with specific filters
//! let mut search_req = default_search();
//! search_req.query.location = Location {
//!     latitude: 47.36667,
//!     longitude: 8.55,
//!     radius: 2000,
//! };
//! search_req.query.monthly_rent = FromTo {
//!     from: Some(1000),
//!     to: Some(2500),
//! };
//! search_req.query.number_of_rooms = FromToFloat {
//!     from: Some(3.0),
//!     to: Some(4.0),
//! };
//! search_req.size = 50; // Get 50 results per page
//!
//! // Execute the search
//! let url = Url::parse(&format!("{}/search/listings", BACKEND_URL))?;
//! let body = serde_json::to_string(&search_req)?;
//! let response = post_url(url, &body).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Legal Disclaimer
//!
//! From the Homegate.ch [disclaimer page](https://www.homegate.ch/c/en/about-us/legal-issues/disclaimer):
//!
//! > All elements, i.e. information, data and layouts on homegate.ch, are the exclusive
//! > and comprehensive property of SMG Swiss Marketplace Group Ltd (in particular, copyrights
//! > and other rights) unless specified otherwise.
//! >
//! > The elements and all information offered on homegate.ch may only be used freely and
//! > without charge for browsing purposes for personal consumption.
//!
//! Use this library responsibly and in compliance with Homegate's terms of service.

pub mod api;
pub mod config;
pub mod error;
pub mod models;

// Re-export commonly used types for convenience
pub use error::{HomegateError, Result};
pub use api::search::{search, Location};
pub use api::request::HomegateClient;
pub use models::realestate::RealEstate;
pub use models::paginated::Paginated;
pub use config::HomegateConfig;
