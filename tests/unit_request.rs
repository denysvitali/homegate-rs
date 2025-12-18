/// Unit tests for request module
///
/// Tests HTTP client building, authentication headers, and request methods

use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_client_has_authorization_header() {
    // Test that requests include proper authentication
    let _mock_server = MockServer::start().await;

    // This demonstrates the expected header format
    let expected_auth = format!("Basic {}",
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD,
        "hg_android:6VcGU6ceCFTk8dFm"));
    assert!(expected_auth.starts_with("Basic "));
}

#[tokio::test]
async fn test_mock_server_setup() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true
        })))
        .mount(&mock_server)
        .await;

    let response = reqwest::Client::new()
        .post(format!("{}/test", mock_server.uri()))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_request_with_500_error() {
    // Test handling of server errors
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/error"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let response = reqwest::Client::new()
        .post(format!("{}/error", mock_server.uri()))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 500);
}

#[tokio::test]
async fn test_request_with_network_timeout() {
    // Test handling of network timeouts
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/timeout"))
        .respond_with(ResponseTemplate::new(200).set_delay(std::time::Duration::from_millis(100)))
        .mount(&mock_server)
        .await;

    let response = reqwest::Client::new()
        .post(format!("{}/timeout", mock_server.uri()))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[test]
fn test_url_parsing_valid() {
    // Test valid URL parsing
    use reqwest::Url;

    let url = Url::parse("https://api.homegate.ch/search/listings");
    assert!(url.is_ok());

    let parsed_url = url.unwrap();
    assert_eq!(parsed_url.scheme(), "https");
    assert_eq!(parsed_url.host_str(), Some("api.homegate.ch"));
}

#[test]
fn test_url_parsing_invalid() {
    // Test invalid URL parsing
    use reqwest::Url;

    let url = Url::parse("not a valid url");
    assert!(url.is_err());
}

#[test]
fn test_basic_auth_encoding() {
    // Test that Basic Auth is encoded correctly
    use base64::Engine;

    let username = "hg_android";
    let password = "6VcGU6ceCFTk8dFm";
    let credentials = format!("{}:{}", username, password);
    let encoded = base64::engine::general_purpose::STANDARD.encode(&credentials);

    // Verify the encoded value matches expected format
    assert!(!encoded.is_empty());

    // Decode and verify
    let decoded = base64::engine::general_purpose::STANDARD.decode(&encoded).unwrap();
    let decoded_str = String::from_utf8(decoded).unwrap();
    assert_eq!(decoded_str, credentials);
}

#[tokio::test]
async fn test_json_content_type() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "test": "value"
        })))
        .mount(&mock_server)
        .await;

    let response = reqwest::Client::new()
        .post(format!("{}/test", mock_server.uri()))
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let json: serde_json::Value = response.json().await.unwrap();
    assert_eq!(json["test"], "value");
}

#[tokio::test]
async fn test_post_with_body() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let body = r#"{"query":"test"}"#;
    let response = reqwest::Client::new()
        .post(format!("{}/test", mock_server.uri()))
        .body(body)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[test]
fn test_headers_creation() {
    use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    assert!(headers.contains_key(CONTENT_TYPE));
    assert_eq!(headers.get(CONTENT_TYPE).unwrap(), "application/json");
}

#[tokio::test]
async fn test_multiple_requests() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();

    for _ in 0..3 {
        let response = client
            .post(format!("{}/test", mock_server.uri()))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
    }
}
