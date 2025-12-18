use std::fmt;

/// Custom error type for the Homegate library.
///
/// This enum represents all possible errors that can occur when interacting
/// with the Homegate API or processing responses.
#[derive(Debug)]
pub enum HomegateError {
    /// HTTP request failed
    Request(reqwest::Error),
    /// Middleware error (retry, timeout, etc.)
    Middleware(String),
    /// Failed to parse JSON response
    ParseError(serde_json::Error),
    /// Invalid header value provided
    InvalidHeader(String),
    /// Invalid URL construction
    InvalidUrl(url::ParseError),
    /// Input validation failed
    ValidationError(String),
}

impl fmt::Display for HomegateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HomegateError::Request(e) => write!(f, "HTTP request failed: {}", e),
            HomegateError::Middleware(s) => write!(f, "Middleware error: {}", s),
            HomegateError::ParseError(e) => write!(f, "Failed to parse response: {}", e),
            HomegateError::InvalidHeader(s) => write!(f, "Invalid header value: {}", s),
            HomegateError::InvalidUrl(e) => write!(f, "Invalid URL: {}", e),
            HomegateError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for HomegateError {}

impl From<reqwest::Error> for HomegateError {
    fn from(err: reqwest::Error) -> Self {
        HomegateError::Request(err)
    }
}

impl From<serde_json::Error> for HomegateError {
    fn from(err: serde_json::Error) -> Self {
        HomegateError::ParseError(err)
    }
}

impl From<url::ParseError> for HomegateError {
    fn from(err: url::ParseError) -> Self {
        HomegateError::InvalidUrl(err)
    }
}

impl From<reqwest_middleware::Error> for HomegateError {
    fn from(err: reqwest_middleware::Error) -> Self {
        // reqwest_middleware::Error can wrap different error types
        // For now, we'll treat all middleware errors as generic middleware errors
        // This includes retry exhaustion, timeout, etc.
        if err.is_request() {
            // If there's an underlying reqwest error, we can't easily extract it
            // without consuming the error, so we convert to string
            HomegateError::Middleware(format!("Request error: {}", err))
        } else {
            HomegateError::Middleware(err.to_string())
        }
    }
}

/// Type alias for Result with HomegateError as the error type
pub type Result<T> = std::result::Result<T, HomegateError>;
