//! Configuration options for the Homegate API client.
//!
//! This module provides configuration structures for customizing client behavior,
//! including backend URL, timeouts, and retry settings.

use std::time::Duration;

/// Configuration for the Homegate API client.
///
/// This structure allows you to customize various aspects of the API client's behavior,
/// including the backend URL, request timeout, and retry policy.
///
/// # Examples
///
/// ```
/// use homegate::config::HomegateConfig;
/// use std::time::Duration;
///
/// // Use default configuration
/// let config = HomegateConfig::default();
///
/// // Or customize the configuration
/// let custom_config = HomegateConfig {
///     backend_url: "https://api.homegate.ch".to_string(),
///     timeout: Duration::from_secs(60),
///     max_retries: 5,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct HomegateConfig {
    /// The base URL for the Homegate API backend
    pub backend_url: String,

    /// Request timeout duration
    ///
    /// This is the maximum time to wait for a complete HTTP request/response cycle.
    /// If the timeout is exceeded, the request will fail.
    pub timeout: Duration,

    /// Maximum number of retry attempts for transient errors
    ///
    /// When a request fails due to transient errors (like network issues or 5xx responses),
    /// the client will automatically retry up to this many times using exponential backoff.
    pub max_retries: u32,
}

impl Default for HomegateConfig {
    /// Creates a default configuration.
    ///
    /// # Default Values
    ///
    /// - `backend_url`: `https://api.homegate.ch`
    /// - `timeout`: 30 seconds
    /// - `max_retries`: 3
    fn default() -> Self {
        Self {
            backend_url: crate::api::BACKEND_URL.to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
        }
    }
}

impl HomegateConfig {
    /// Creates a new configuration with custom settings.
    ///
    /// # Arguments
    ///
    /// * `backend_url` - The base URL for the Homegate API
    /// * `timeout` - Request timeout duration
    /// * `max_retries` - Maximum number of retry attempts
    ///
    /// # Examples
    ///
    /// ```
    /// use homegate::config::HomegateConfig;
    /// use std::time::Duration;
    ///
    /// let config = HomegateConfig::new(
    ///     "https://api.homegate.ch",
    ///     Duration::from_secs(45),
    ///     5
    /// );
    /// ```
    pub fn new(backend_url: impl Into<String>, timeout: Duration, max_retries: u32) -> Self {
        Self {
            backend_url: backend_url.into(),
            timeout,
            max_retries,
        }
    }

    /// Creates a configuration for testing with shorter timeouts.
    ///
    /// # Default Values for Testing
    ///
    /// - `backend_url`: `https://api.homegate.ch`
    /// - `timeout`: 10 seconds
    /// - `max_retries`: 1
    #[cfg(test)]
    pub fn for_testing() -> Self {
        Self {
            backend_url: crate::api::BACKEND_URL.to_string(),
            timeout: Duration::from_secs(10),
            max_retries: 1,
        }
    }
}
