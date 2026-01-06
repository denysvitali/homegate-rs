/// Common test utilities for homegate-rs tests
///
/// This module provides shared functionality for setting up mock servers,
/// loading fixtures, and other test utilities.
pub mod fixtures;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Sets up a wiremock server for testing HTTP requests
///
/// # Returns
///
/// A `MockServer` instance that can be used to mock HTTP responses
pub async fn setup_mock_server() -> MockServer {
    MockServer::start().await
}

/// Creates a mock for a successful search request
///
/// # Arguments
///
/// * `server` - The mock server to register the mock with
/// * `response_body` - The JSON response body to return
pub async fn mock_search_request(server: &MockServer, response_body: &str) {
    Mock::given(method("POST"))
        .and(path("/search/listings"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(server)
        .await;
}

/// Creates a mock for a successful GET request
///
/// # Arguments
///
/// * `server` - The mock server to register the mock with
/// * `path_str` - The path to mock
/// * `response_body` - The JSON response body to return
pub async fn mock_get_request(server: &MockServer, path_str: &str, response_body: &str) {
    Mock::given(method("GET"))
        .and(path(path_str))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(server)
        .await;
}

/// Creates a mock that returns an error status code
///
/// # Arguments
///
/// * `server` - The mock server to register the mock with
/// * `method_str` - The HTTP method (GET, POST, etc.)
/// * `path_str` - The path to mock
/// * `status_code` - The error status code to return
pub async fn mock_error_request(
    server: &MockServer,
    method_str: &str,
    path_str: &str,
    status_code: u16,
) {
    Mock::given(method(method_str))
        .and(path(path_str))
        .respond_with(ResponseTemplate::new(status_code))
        .mount(server)
        .await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_setup_mock_server() {
        let server = setup_mock_server().await;
        assert!(!server.uri().is_empty());
    }

    #[tokio::test]
    async fn test_mock_get_request() {
        let server = setup_mock_server().await;
        mock_get_request(&server, "/test", r#"{"success": true}"#).await;

        let response = reqwest::get(&format!("{}/test", server.uri()))
            .await
            .unwrap();
        assert_eq!(response.status(), 200);

        let body = response.text().await.unwrap();
        assert_eq!(body, r#"{"success": true}"#);
    }

    #[tokio::test]
    async fn test_mock_error_request() {
        let server = setup_mock_server().await;
        mock_error_request(&server, "GET", "/error", 500).await;

        let response = reqwest::get(&format!("{}/error", server.uri()))
            .await
            .unwrap();
        assert_eq!(response.status(), 500);
    }
}
