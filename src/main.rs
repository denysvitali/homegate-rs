use std::process;

use clap::{Parser, Subcommand};
use comfy_table::{presets::UTF8_FULL, ContentArrangement, Table};
use url::Url;

use homegate::api::request::HomegateClient;
use homegate::api::search::{default_search, Location};
use homegate::api::BACKEND_URL;
use homegate::models::paginated::parse_search_result;
use homegate::models::realestate::OfferType;

mod mcp;

/// Homegate.ch CLI and MCP server for real estate search
#[derive(Parser, Debug)]
#[command(name = "homegate")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Search for real estate listings (default if no subcommand)
    Search(SearchArgs),
    /// Run as MCP (Model Context Protocol) server
    Serve,
}

/// Arguments for the search command
#[derive(Parser, Debug)]
struct SearchArgs {
    /// Latitude (-90 to 90)
    #[arg(long)]
    lat: f32,

    /// Longitude (-180 to 180)
    #[arg(long)]
    lon: f32,

    /// Search radius in meters (default: 5000, max: 49999)
    #[arg(long, default_value_t = 5000)]
    radius: u32,

    /// Minimum monthly rent/price in CHF
    #[arg(long)]
    min_price: Option<u32>,

    /// Maximum monthly rent/price in CHF
    #[arg(long)]
    max_price: Option<u32>,

    /// Minimum number of rooms (supports fractional values like 2.5, 3.5)
    #[arg(long)]
    min_rooms: Option<f32>,

    /// Maximum number of rooms (supports fractional values like 2.5, 3.5)
    #[arg(long)]
    max_rooms: Option<f32>,

    /// Minimum living space in m²
    #[arg(long)]
    min_space: Option<u32>,

    /// Maximum living space in m²
    #[arg(long)]
    max_space: Option<u32>,

    /// Property categories to include (comma-separated or repeatable)
    /// Examples: APARTMENT, FLAT, STUDIO, VILLA, CHALET, SINGLE_HOUSE
    #[arg(long, value_delimiter = ',')]
    category: Option<Vec<String>>,

    /// Property categories to exclude (comma-separated or repeatable)
    #[arg(long, value_delimiter = ',')]
    exclude_category: Option<Vec<String>>,

    /// Offer type: rent (default)
    #[arg(long, default_value = "rent")]
    offer_type: String,

    /// Page number (1-indexed)
    #[arg(long, default_value_t = 1)]
    page: u32,

    /// Results per page
    #[arg(long, default_value_t = 20)]
    page_size: i32,

    /// Output as JSON instead of table
    #[arg(long)]
    json: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Search(args)) => run_search(args).await,
        Some(Commands::Serve) => run_mcp_server().await,
        None => {
            // If no subcommand, show help
            eprintln!("Usage: homegate <COMMAND>");
            eprintln!();
            eprintln!("Commands:");
            eprintln!("  search  Search for real estate listings");
            eprintln!("  serve   Run as MCP server");
            eprintln!();
            eprintln!("Run 'homegate --help' for more information");
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

async fn run_mcp_server() -> Result<(), Box<dyn std::error::Error>> {
    use rmcp::transport::stdio;
    use rmcp::ServiceExt;

    let server = mcp::HomegateServer::new();
    let transport = stdio();
    server.serve(transport).await?.waiting().await?;
    Ok(())
}

async fn run_search(args: SearchArgs) -> Result<(), Box<dyn std::error::Error>> {
    // Validate location
    let location = Location {
        latitude: args.lat,
        longitude: args.lon,
        radius: args.radius,
    };
    location
        .validate()
        .map_err(|e| format!("Invalid location: {}", e))?;

    // Build search request from defaults
    let mut search_request = default_search();

    // Override location
    search_request.query.location = location;

    // Override price filter if specified
    if args.min_price.is_some() || args.max_price.is_some() {
        if args.min_price.is_some() {
            search_request.query.monthly_rent.from = args.min_price;
        }
        if args.max_price.is_some() {
            search_request.query.monthly_rent.to = args.max_price;
        }
    }
    search_request
        .query
        .monthly_rent
        .validate()
        .map_err(|e| format!("Invalid price range: {}", e))?;

    // Override rooms filter if specified
    if args.min_rooms.is_some() || args.max_rooms.is_some() {
        if args.min_rooms.is_some() {
            search_request.query.number_of_rooms.from = args.min_rooms;
        }
        if args.max_rooms.is_some() {
            search_request.query.number_of_rooms.to = args.max_rooms;
        }
    }
    search_request
        .query
        .number_of_rooms
        .validate()
        .map_err(|e| format!("Invalid rooms range: {}", e))?;

    // Override living space filter if specified
    if args.min_space.is_some() || args.max_space.is_some() {
        if args.min_space.is_some() {
            search_request.query.living_space.from = args.min_space;
        }
        if args.max_space.is_some() {
            search_request.query.living_space.to = args.max_space;
        }
    }
    search_request
        .query
        .living_space
        .validate()
        .map_err(|e| format!("Invalid space range: {}", e))?;

    // Override categories if specified
    if let Some(categories) = args.category {
        search_request.query.categories = categories
            .into_iter()
            .map(|c| c.to_uppercase().replace('-', "_"))
            .collect();
    }

    // Override exclude categories if specified
    if let Some(exclude) = args.exclude_category {
        search_request.query.exclude_categories = exclude
            .into_iter()
            .map(|c| c.to_uppercase().replace('-', "_"))
            .collect();
    }

    // Override offer type
    search_request.query.offer_type = match args.offer_type.to_lowercase().as_str() {
        "rent" => OfferType::RENT,
        other => return Err(format!("Unknown offer type: {}. Supported: rent", other).into()),
    };

    // Pagination
    search_request.size = args.page_size;
    search_request.from = ((args.page - 1) as i32) * args.page_size;

    // Execute search
    let client = HomegateClient::new()?;
    let url = Url::parse(&format!("{}/search/listings", BACKEND_URL))?;
    let body = serde_json::to_string(&search_request)?;
    let resp = client.post_url(url, &body).await?;
    let text = resp.text().await?;
    let results = parse_search_result(&text)?;

    // Output results
    if args.json {
        println!("{}", serde_json::to_string_pretty(&results)?);
    } else {
        print_table(&results, args.page, args.page_size);
    }

    Ok(())
}

fn print_table(results: &homegate::Paginated<homegate::RealEstate>, page: u32, page_size: i32) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["ID", "Address", "Rooms", "Space", "Price (CHF)"]);

    for item in &results.results {
        let listing = &item.listing;

        // Build address string
        let address = format!(
            "{}, {} {}",
            listing.address.street.as_deref().unwrap_or("-"),
            listing.address.postal_code,
            listing.address.locality.as_deref().unwrap_or("")
        );

        // Format rooms
        let rooms = format!("{:.1}", listing.characteristics.number_of_rooms);

        // Format space
        let space = format!("{} m²", listing.characteristics.living_space);

        // Format price
        let price = if let Some(ref rent) = listing.prices.rent {
            if let Some(gross) = rent.gross {
                format!("{}/mo", gross)
            } else if let Some(net) = rent.net {
                format!("{}/mo (net)", net)
            } else {
                "-".to_string()
            }
        } else if let Some(ref buy) = listing.prices.buy {
            if let Some(gross) = buy.gross {
                format!("{}", gross)
            } else {
                "-".to_string()
            }
        } else {
            "-".to_string()
        };

        table.add_row(vec![&listing.id, &address, &rooms, &space, &price]);
    }

    println!("{table}");

    let total_pages = (results.total as f64 / page_size as f64).ceil() as u32;
    let start = ((page - 1) * page_size as u32) + 1;
    let end = std::cmp::min(start + results.results.len() as u32 - 1, results.total);

    if results.total > 0 {
        println!(
            "Page {} of {} ({}-{} of {} results)",
            page, total_pages, start, end, results.total
        );
    } else {
        println!("No results found");
    }
}
