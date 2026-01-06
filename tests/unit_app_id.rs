/// Unit tests for app_id module
///
/// Tests HMAC calculation, app ID generation, and version formatting
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

// Helper function to create a test datetime
fn create_datetime(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
) -> NaiveDateTime {
    NaiveDateTime::new(
        NaiveDate::from_ymd_opt(year, month, day).unwrap(),
        NaiveTime::from_hms_opt(hour, min, sec).unwrap(),
    )
}

#[test]
fn test_calculate_app_id_known_value() {
    // Test with the known example from existing tests
    let dt = create_datetime(2022, 1, 25, 1, 30, 56);
    let app_id = homegate::api::app_id::calculate_app_id(&dt);
    assert_eq!("1926888397", app_id);
}

#[test]
fn test_calculate_app_id_different_timestamps() {
    // Test with different timestamps to ensure deterministic behavior
    let dt1 = create_datetime(2023, 6, 15, 10, 0, 0);
    let dt2 = create_datetime(2023, 6, 15, 10, 0, 0);

    let app_id1 = homegate::api::app_id::calculate_app_id(&dt1);
    let app_id2 = homegate::api::app_id::calculate_app_id(&dt2);

    assert_eq!(
        app_id1, app_id2,
        "Same timestamp should produce same app ID"
    );
}

#[test]
fn test_calculate_app_id_different_minutes_same_ceiling() {
    // Test that timestamps within the same minute ceiling produce the same ID
    let dt1 = create_datetime(2023, 6, 15, 10, 0, 10);
    let dt2 = create_datetime(2023, 6, 15, 10, 0, 50);

    let app_id1 = homegate::api::app_id::calculate_app_id(&dt1);
    let app_id2 = homegate::api::app_id::calculate_app_id(&dt2);

    assert_eq!(
        app_id1, app_id2,
        "Timestamps in same minute should produce same app ID"
    );
}

#[test]
fn test_calculate_app_id_different_minutes() {
    // Test that different minutes produce different IDs
    let dt1 = create_datetime(2023, 6, 15, 10, 0, 0);
    let dt2 = create_datetime(2023, 6, 15, 10, 1, 0);

    let app_id1 = homegate::api::app_id::calculate_app_id(&dt1);
    let app_id2 = homegate::api::app_id::calculate_app_id(&dt2);

    assert_ne!(
        app_id1, app_id2,
        "Different minutes should produce different app IDs"
    );
}

#[test]
fn test_calculate_app_id_epoch() {
    // Test with Unix epoch
    let dt = create_datetime(1970, 1, 1, 0, 0, 0);
    let app_id = homegate::api::app_id::calculate_app_id(&dt);

    // App ID should be a valid string representation of a number
    assert!(
        app_id.parse::<i64>().is_ok(),
        "App ID should be parseable as a number"
    );
}

#[test]
fn test_calculate_app_id_future_timestamp() {
    // Test with a future timestamp
    let dt = create_datetime(2030, 12, 31, 23, 59, 59);
    let app_id = homegate::api::app_id::calculate_app_id(&dt);

    // App ID should be a valid string representation of a number
    assert!(
        app_id.parse::<i64>().is_ok(),
        "App ID should be parseable as a number"
    );
    assert!(!app_id.is_empty(), "App ID should not be empty");
}

#[test]
fn test_app_version_format() {
    let version = homegate::api::app_id::app_version();

    // Check format: "Homegate/12.6.0/12060003/Android/30"
    assert!(
        version.starts_with("Homegate/"),
        "Version should start with 'Homegate/'"
    );
    assert!(
        version.contains("/Android/"),
        "Version should contain '/Android/'"
    );
    assert!(version.ends_with("/30"), "Version should end with '/30'");
}

#[test]
fn test_app_version_consistency() {
    // Version should always return the same value
    let version1 = homegate::api::app_id::app_version();
    let version2 = homegate::api::app_id::app_version();

    assert_eq!(
        version1, version2,
        "app_version should return consistent results"
    );
}
