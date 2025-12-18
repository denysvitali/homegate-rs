# UNOFFICIAL Homegate Library for Rust

<p align="center">
  <img
    src="./docs/logo.png"
    alt="homegate.ch logo"
    height="60"
  />
</p>

[![Crates.io](https://img.shields.io/crates/v/homegate.svg)](https://crates.io/crates/homegate)
[![Documentation](https://docs.rs/homegate/badge.svg)](https://docs.rs/homegate)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Description

This is an unofficial Rust library that lets you interact with the [Homegate.ch](https://homegate.ch) real estate platform. It provides a simple interface to search for rental and sale properties in Switzerland.

**⚠️ Warning:** This library uses unofficial APIs extracted from the Homegate mobile application. Your account might get banned for excessive scraping. Use responsibly!

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
homegate = "1.0"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use homegate::api::search::{search, Location};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Search for properties in Zurich within 1km radius
    let results = search(&Location {
        latitude: 47.36667,
        longitude: 8.55,
        radius: 1000, // meters
    }).await?;

    println!("Found {} total listings", results.total);

    for real_estate in results.results {
        let listing = real_estate.listing;
        println!("Property ID: {}", listing.id);
        println!("  Address: {:?}", listing.address.locality);
        println!("  Rooms: {}", listing.characteristics.number_of_rooms);
        println!("  Size: {}m²", listing.characteristics.living_space);

        if let Some(rent) = listing.prices.rent {
            if let Some(gross) = rent.gross {
                println!("  Rent: {} CHF/month", gross);
            }
        }
    }

    Ok(())
}
```

## Features

- 🔍 **Property Search** - Search for real estate by location, price, size, and category
- 📊 **Rich Data Models** - Comprehensive structures for listings, addresses, and prices
- 🔒 **Type-Safe** - Strongly typed API with full serde support
- ⚡ **Async/Await** - Built on tokio and reqwest for async HTTP
- 📄 **Pagination** - Support for paginated search results

## Advanced Usage

### Custom Search with Filters

```rust
use homegate::api::search::{default_search, Location, FromTo};
use homegate::models::listing::Category;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut search_req = default_search();

    // Set location
    search_req.query.location = Location {
        latitude: 47.36667,
        longitude: 8.55,
        radius: 2000,
    };

    // Filter by price
    search_req.query.monthly_rent = FromTo {
        from: Some(1000),
        to: Some(2500),
    };

    // Filter by number of rooms
    search_req.query.number_of_rooms = FromTo {
        from: Some(3),
        to: Some(4),
    };

    // Filter by living space
    search_req.query.living_space = FromTo {
        from: Some(80),
        to: None,
    };

    // Results per page
    search_req.size = 50;

    // Execute search (you'll need to call the API directly here)
    // See documentation for full example

    Ok(())
}
```

### Property Categories

The library supports various property categories:

**Apartments:**
- Apartment, Flat, Studio, Loft
- Maisonette, Duplex, AtticFlat, RoofFlat
- TerraceFlat, BachelorFlat, SingleRoom

**Houses:**
- SingleHouse, RowHouse, TerraceHouse
- Villa, Chalet, Rustico
- FarmHouse, Castle, BifamiliarHouse

**Other:**
- HobbyRoom, CellarCompartment, AtticCompartment

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/homegate).

## Disclaimer

This repository and library has been created for **educational purposes and private use only**.

The data accessed through this library is the property of Homegate.ch (SMG Swiss Marketplace Group Ltd), and you are **not allowed to re-sell it or re-use it publicly**.

Following is an excerpt from the [Homegate.ch disclaimer page](https://www.homegate.ch/c/en/about-us/legal-issues/disclaimer):

> ### Copyright/Ban on Republication
>
> All elements, i.e. information, data and layouts on homegate.ch, are the exclusive and comprehensive property of SMG Swiss Marketplace Group Ltd (in particular, copyrights and other rights) unless specified otherwise.
>
> The elements and all information offered on homegate.ch may only be used freely and without charge for browsing purposes for personal consumption. By accessing homegate.ch, the user expressly agrees not to copy, publish or provide access to the elements and information offered, in any form whatsoever, particularly on the World Wide Web.
>
> The user must therefore obtain the prior written consent of SMG Swiss Marketplace Group Ltd, without exception, for any republication of elements in any medium whatsoever, in particular for providing public access to these elements on a URL other than homegate.ch.

**Use this library responsibly and in compliance with Homegate's terms of service.**

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

Denys Vitali - [denys@denv.it](mailto:denys@denv.it)
