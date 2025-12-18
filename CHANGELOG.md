# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive rustdoc documentation for all public API items
- Module-level documentation for api and models modules
- Detailed documentation for search functionality and data models
- Code examples in documentation
- Enhanced README.md with usage examples and feature list

### Changed
- Improved error handling with custom error types
- Updated API to use Result type alias for better error handling
- Enhanced search functionality with better type safety

## [1.0.1] - 2024

### Changed
- Updated dependencies to latest versions
- Fixed tests
- Minor bug fixes

### Fixed
- Removed geo module
- Fixed typos in documentation

## [1.0.0] - Initial Release

### Added
- Initial implementation of Homegate API client
- Search functionality for real estate listings
- Support for location-based searches
- Property filtering by:
  - Categories (apartments, houses, rooms, etc.)
  - Price range
  - Living space
  - Number of rooms
- Pagination support for search results
- Type-safe models for:
  - Real estate listings
  - Addresses and geographic coordinates
  - Prices and property characteristics
  - Localization data
- Async/await support using tokio
- HTTP client with proper authentication headers
- App ID generation for API authentication

### Security
- Uses extracted credentials from official mobile app
- Implements proper authentication headers
- HTTPS-only communication with backend

### Documentation
- Basic README with installation instructions
- Legal disclaimer from Homegate.ch
- Project description and usage warnings

### Dependencies
- reqwest for HTTP requests
- serde/serde_json for JSON serialization
- tokio for async runtime
- chrono for time handling
- base64, hmac, sha2 for authentication

[Unreleased]: https://github.com/denysvitali/homegate-rs/compare/v1.0.1...HEAD
[1.0.1]: https://github.com/denysvitali/homegate-rs/releases/tag/v1.0.1
[1.0.0]: https://github.com/denysvitali/homegate-rs/releases/tag/v1.0.0
