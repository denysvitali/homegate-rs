//! HTTP request handling for the Homegate API.
//!
//! This module provides low-level HTTP client functionality with proper authentication
//! headers and app identification for communicating with the Homegate backend.

use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use reqwest::header;
use reqwest::header::HeaderValue;
use reqwest::{Client, ClientBuilder, Response, Url};
use reqwest_middleware::{ClientBuilder as MiddlewareClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

use crate::api::app_id::{app_version, calculate_app_id};
use crate::api::{API_PASSWORD, API_USERNAME, USER_AGENT};

/// HTTP client for the Homegate API with persistent connection pooling.
///
/// This struct maintains a configured reqwest `ClientWithMiddleware` instance that can be reused
/// across multiple requests, avoiding the overhead of rebuilding the client and
/// connection pool for each request.
///
/// # Performance Benefits
///
/// - Reuses TCP connections via connection pooling
/// - Avoids recreating authentication headers on each request
/// - Reduces allocation overhead from repeated client construction
/// - Includes automatic retry logic for transient failures
///
/// # Examples
///
/// ```no_run
/// use homegate::api::request::HomegateClient;
/// use reqwest::Url;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = HomegateClient::new()?;
/// let url = Url::parse("https://api.homegate.ch/search/listings")?;
/// let response = client.post_url(url, r#"{"query": {}}"#).await?;
/// # Ok(())
/// # }
/// ```
pub struct HomegateClient {
    client: ClientWithMiddleware,
}

impl HomegateClient {
    /// Creates a new Homegate API client with default retry settings (3 retries).
    ///
    /// This initializes a reqwest client with all necessary authentication headers
    /// including Basic Auth, app identification, and user agent.
    ///
    /// # Returns
    ///
    /// Returns a configured `HomegateClient` on success, or a `HomegateError` if
    /// client construction fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use homegate::api::request::HomegateClient;
    ///
    /// # fn example() -> homegate::Result<()> {
    /// let client = HomegateClient::new()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> crate::Result<Self> {
        Self::with_retries(3)
    }

    /// Creates a new Homegate API client with custom retry settings.
    ///
    /// # Arguments
    ///
    /// * `max_retries` - Maximum number of retry attempts for transient errors
    ///
    /// # Returns
    ///
    /// Returns a configured `HomegateClient` on success, or a `HomegateError` if
    /// client construction fails.
    pub fn with_retries(max_retries: u32) -> crate::Result<Self> {
        let client = build_client(max_retries)?;
        Ok(Self { client })
    }

    /// Sends an authenticated POST request to the specified URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the POST request to
    /// * `body` - The request body as a string (typically JSON)
    ///
    /// # Returns
    ///
    /// Returns the HTTP `Response` on success, or a `HomegateError` if the request fails.
    #[tracing::instrument(level = "info", skip(self, body), fields(url = %url))]
    pub async fn post_url(&self, url: Url, body: &str) -> crate::Result<Response> {
        tracing::info!("Sending POST request via HomegateClient");
        let req = self.client.post(url).body(body.to_string()).build()?;
        Ok(self.client.execute(req).await?)
    }

    /// Searches for real estate listings at the specified location.
    ///
    /// This is a convenience method that wraps the search API with proper error handling
    /// and response parsing.
    ///
    /// # Arguments
    ///
    /// * `location` - Geographic location and search radius
    ///
    /// # Returns
    ///
    /// Returns a `Paginated<RealEstate>` containing search results.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use homegate::api::request::HomegateClient;
    /// use homegate::api::search::Location;
    ///
    /// # async fn example() -> homegate::Result<()> {
    /// let client = HomegateClient::new()?;
    /// let location = Location {
    ///     latitude: 47.36667,
    ///     longitude: 8.55,
    ///     radius: 1000,
    /// };
    /// let results = client.search(&location).await?;
    /// println!("Found {} listings", results.total);
    /// # Ok(())
    /// # }
    /// ```
    #[tracing::instrument(level = "info", skip(self), fields(lat = %location.latitude, lon = %location.longitude, radius = %location.radius))]
    pub async fn search(
        &self,
        location: &crate::api::search::Location,
    ) -> crate::Result<crate::models::paginated::Paginated<crate::models::realestate::RealEstate>>
    {
        use crate::api::search::default_search;
        use crate::api::BACKEND_URL;
        use crate::models::paginated::parse_search_result;

        // Validate location parameters
        location
            .validate()
            .map_err(crate::HomegateError::ValidationError)?;

        tracing::info!("Searching for real estate listings");
        let url: Url = Url::parse(&format!("{}{}", BACKEND_URL, "/search/listings"))?;

        let mut search_request = default_search();
        search_request.query.location = location.clone();

        // Validate FromTo ranges in the query
        search_request
            .query
            .living_space
            .validate()
            .map_err(|e| crate::HomegateError::ValidationError(format!("living_space: {}", e)))?;
        search_request
            .query
            .monthly_rent
            .validate()
            .map_err(|e| crate::HomegateError::ValidationError(format!("monthly_rent: {}", e)))?;
        search_request
            .query
            .number_of_rooms
            .validate()
            .map_err(|e| {
                crate::HomegateError::ValidationError(format!("number_of_rooms: {}", e))
            })?;

        let search_request_json = serde_json::to_string(&search_request)?;

        let resp = self.post_url(url, &search_request_json).await?;
        let resp_text = resp.text().await?;
        let r = parse_search_result(&resp_text)?;

        tracing::info!("Search completed successfully, found {} results", r.total);
        Ok(r)
    }
}

impl Default for HomegateClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default HomegateClient")
    }
}

/// Builds an authenticated HTTP client with required Homegate API headers.
///
/// This function creates a reqwest Client configured with all necessary authentication
/// headers including Basic Auth, app identification, and user agent.
///
/// # Arguments
///
/// * `max_retries` - Maximum number of retry attempts for transient errors
///
/// # Returns
///
/// Returns a configured `ClientWithMiddleware` on success, or an `Error` if client construction fails.
#[tracing::instrument(level = "debug")]
fn build_client(max_retries: u32) -> crate::Result<ClientWithMiddleware> {
    tracing::debug!("Building HTTP client with retry middleware");
    let client_builder: ClientBuilder = Client::builder();
    let mut default_headers = header::HeaderMap::new();

    let key = general_purpose::STANDARD.encode(format!("{}:{}", API_USERNAME, API_PASSWORD));
    let app_id = calculate_app_id(&Utc::now().naive_utc());

    const APPL_JSON: &str = "application/json";

    default_headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Basic {}", key))
            .map_err(|e| crate::HomegateError::InvalidHeader(e.to_string()))?,
    );
    default_headers.insert(header::ACCEPT, HeaderValue::from_static(APPL_JSON));
    default_headers.insert(
        "X-App-Id",
        app_id.parse().map_err(|e: header::InvalidHeaderValue| {
            crate::HomegateError::InvalidHeader(e.to_string())
        })?,
    );
    default_headers.insert(
        "X-App-Version",
        app_version()
            .parse()
            .map_err(|e: header::InvalidHeaderValue| {
                crate::HomegateError::InvalidHeader(e.to_string())
            })?,
    );
    default_headers.insert(header::USER_AGENT, HeaderValue::from_static(USER_AGENT)); // Not a typo!
    default_headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(APPL_JSON));

    let client = client_builder.default_headers(default_headers).build()?;

    // Configure exponential backoff retry policy
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(max_retries);
    let retry_middleware = RetryTransientMiddleware::new_with_policy(retry_policy);

    let client_with_middleware = MiddlewareClientBuilder::new(client)
        .with(retry_middleware)
        .build();

    Ok(client_with_middleware)
}

/// Sends an authenticated POST request to the specified URL.
///
/// This function creates an HTTP POST request with the provided body and sends it
/// using an authenticated client with retry middleware.
///
/// # Arguments
///
/// * `url` - The URL to send the POST request to
/// * `body` - The request body as a string (typically JSON)
///
/// # Returns
///
/// Returns the HTTP `Response` on success, or an `Error` if the request fails.
///
/// # Examples
///
/// ```no_run
/// use reqwest::Url;
/// use homegate::api::request::post_url;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let url = Url::parse("https://api.homegate.ch/search/listings")?;
/// let response = post_url(url, r#"{"query": {}}"#).await?;
/// # Ok(())
/// # }
/// ```
#[tracing::instrument(level = "info", skip(body), fields(url = %url))]
pub async fn post_url(url: Url, body: &str) -> crate::Result<Response> {
    tracing::info!("Sending POST request");
    let c = build_client(3)?; // Default 3 retries
    let req = c.post(url).body(body.to_string()).build()?;
    c.execute(req).await.map_err(Into::into)
}

