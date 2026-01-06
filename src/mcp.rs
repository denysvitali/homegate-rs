//! MCP (Model Context Protocol) server implementation for Homegate.
//!
//! This module provides an MCP server that exposes the Homegate search
//! functionality as a tool that can be called by AI assistants.

use rmcp::{
    handler::server::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content, Implementation, ServerInfo},
    schemars::JsonSchema,
    tool, tool_router, ErrorData as McpError,
};
use serde::{Deserialize, Serialize};
use url::Url;

use homegate::api::request::HomegateClient;
use homegate::api::search::{default_search, Location};
use homegate::api::BACKEND_URL;
use homegate::models::paginated::parse_search_result;

/// Default search radius in meters
fn default_radius() -> u32 {
    5000
}

/// Default page number
fn default_page() -> u32 {
    1
}

/// Default page size
fn default_page_size() -> i32 {
    20
}

/// Parameters for the search tool
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchParams {
    /// Latitude in degrees (-90 to 90)
    pub latitude: f32,
    /// Longitude in degrees (-180 to 180)
    pub longitude: f32,
    /// Search radius in meters (default: 5000, max: 49999)
    #[serde(default = "default_radius")]
    pub radius: u32,
    /// Minimum monthly rent/price in CHF
    pub min_price: Option<u32>,
    /// Maximum monthly rent/price in CHF
    pub max_price: Option<u32>,
    /// Minimum number of rooms (supports fractional values like 2.5, 3.5)
    pub min_rooms: Option<f32>,
    /// Maximum number of rooms (supports fractional values like 2.5, 3.5)
    pub max_rooms: Option<f32>,
    /// Minimum living space in square meters
    pub min_space: Option<u32>,
    /// Maximum living space in square meters
    pub max_space: Option<u32>,
    /// Property categories to include (e.g., APARTMENT, STUDIO, VILLA)
    pub categories: Option<Vec<String>>,
    /// Page number (1-indexed, default: 1)
    #[serde(default = "default_page")]
    pub page: u32,
    /// Results per page (default: 20)
    #[serde(default = "default_page_size")]
    pub page_size: i32,
}

/// Simplified listing for MCP response
#[derive(Debug, Serialize, JsonSchema)]
pub struct ListingResult {
    /// Listing ID
    pub id: String,
    /// Street address
    pub street: Option<String>,
    /// Postal code
    pub postal_code: String,
    /// City/locality
    pub locality: Option<String>,
    /// Number of rooms
    pub rooms: f32,
    /// Living space in square meters
    pub living_space: u32,
    /// Monthly rent (gross) in CHF
    pub price_gross: Option<u32>,
    /// Monthly rent (net) in CHF
    pub price_net: Option<u32>,
}

/// Search results response
#[derive(Debug, Serialize, JsonSchema)]
pub struct SearchResult {
    /// Total number of results
    pub total: u32,
    /// Current page
    pub page: u32,
    /// Total pages
    pub total_pages: u32,
    /// Listings on this page
    pub listings: Vec<ListingResult>,
}

/// MCP server for Homegate real estate search
#[derive(Clone)]
pub struct HomegateServer {
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl HomegateServer {
    /// Create a new Homegate MCP server
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Search for real estate listings on Homegate.ch
    #[tool(
        description = "Search for real estate listings (apartments, houses, etc.) on Homegate.ch in Switzerland. Returns property details including address, rooms, size, and price."
    )]
    async fn search(
        &self,
        Parameters(params): Parameters<SearchParams>,
    ) -> Result<CallToolResult, McpError> {
        // Validate location
        let location = Location {
            latitude: params.latitude,
            longitude: params.longitude,
            radius: params.radius,
        };

        if let Err(e) = location.validate() {
            return Err(McpError::invalid_params(e, None));
        }

        // Build search request
        let mut search_request = default_search();
        search_request.query.location = location;

        // Override filters if provided
        if let Some(min) = params.min_price {
            search_request.query.monthly_rent.from = Some(min);
        }
        if let Some(max) = params.max_price {
            search_request.query.monthly_rent.to = Some(max);
        }
        if let Some(min) = params.min_rooms {
            search_request.query.number_of_rooms.from = Some(min);
        }
        if let Some(max) = params.max_rooms {
            search_request.query.number_of_rooms.to = Some(max);
        }
        if let Some(min) = params.min_space {
            search_request.query.living_space.from = Some(min);
        }
        if let Some(max) = params.max_space {
            search_request.query.living_space.to = Some(max);
        }
        if let Some(categories) = params.categories {
            search_request.query.categories = categories
                .into_iter()
                .map(|c| c.to_uppercase().replace('-', "_"))
                .collect();
        }

        // Pagination
        search_request.size = params.page_size;
        search_request.from = ((params.page - 1) as i32) * params.page_size;

        // Execute search
        let client = HomegateClient::new().map_err(|e| {
            McpError::internal_error(format!("Failed to create client: {}", e), None)
        })?;

        let url = Url::parse(&format!("{}/search/listings", BACKEND_URL))
            .map_err(|e| McpError::internal_error(format!("Invalid URL: {}", e), None))?;

        let body = serde_json::to_string(&search_request)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        let resp = client
            .post_url(url, &body)
            .await
            .map_err(|e| McpError::internal_error(format!("Request failed: {}", e), None))?;

        let text = resp.text().await.map_err(|e| {
            McpError::internal_error(format!("Failed to read response: {}", e), None)
        })?;

        let results = parse_search_result(&text).map_err(|e| {
            McpError::internal_error(format!("Failed to parse response: {}", e), None)
        })?;

        // Convert to simplified response
        let listings: Vec<ListingResult> = results
            .results
            .iter()
            .map(|r| {
                let listing = &r.listing;
                ListingResult {
                    id: listing.id.clone(),
                    street: listing.address.street.clone(),
                    postal_code: listing.address.postal_code.clone(),
                    locality: listing.address.locality.clone(),
                    rooms: listing.characteristics.number_of_rooms,
                    living_space: listing.characteristics.living_space,
                    price_gross: listing.prices.rent.as_ref().and_then(|p| p.gross),
                    price_net: listing.prices.rent.as_ref().and_then(|p| p.net),
                }
            })
            .collect();

        let total_pages = (results.total as f64 / params.page_size as f64).ceil() as u32;

        let search_result = SearchResult {
            total: results.total,
            page: params.page,
            total_pages,
            listings,
        };

        let json = serde_json::to_string_pretty(&search_result)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}

impl Default for HomegateServer {
    fn default() -> Self {
        Self::new()
    }
}

impl rmcp::handler::server::ServerHandler for HomegateServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: "homegate".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                title: None,
                website_url: None,
                icons: None,
            },
            ..Default::default()
        }
    }
}
