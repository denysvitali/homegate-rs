//! API module providing access to Homegate's backend services.
//!
//! This module contains the core API functionality for interacting with the unofficial
//! Homegate API, including request handling and search capabilities.

pub mod app_id;
pub mod request;
pub mod search;

/// The base URL for Homegate's API backend.
///
/// This points to the production API endpoint used by the official Homegate mobile application.
pub static BACKEND_URL: &str = "https://api.homegate.ch";
// pub static BACKEND_URL: &str = "http://127.0.0.1:1234";

/// API username for authentication with the Homegate backend.
///
/// This is extracted from the official Android application and is used for HTTP Basic Authentication.
pub(crate) static API_USERNAME: &str = "hg_android";

/// API password for authentication with the Homegate backend.
///
/// This is extracted from the official Android application and is used for HTTP Basic Authentication.
pub(crate) static API_PASSWORD: &str = "6VcGU6ceCFTk8dFm";

/// Secret key used for generating app authentication signatures.
///
/// This byte array is used in conjunction with the app ID generation process
/// to authenticate requests to the Homegate API.
pub(crate) static SECRET: [u8; 21] = [
    65, 66, 117, 84, 90, 114, 99, 84, 71, 75, 78, 52, 65, 119, 106, 72, 101, 100, 51, 72, 106,
];

/// User agent string identifying requests as coming from the Android app.
///
/// This header is required by the API to identify the client application.
pub(crate) static USER_AGENT: &str = "homegate.ch App Android";
