use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::env;
use assert_cmd::assert::OutputAssertExt;

const MOCK_VALID_ZIP_CODE: &str = "90210"; // Using a valid zip code for live API tests
const MOCK_INVALID_ZIP_CODE: &str = "00000"; // Invalid zip code for error tests

#[test]
fn test_qwx_success_output_live_api() {

    let mut cmd = assert_cmd::Command::cargo_bin("qwx").unwrap();
    cmd.env("OPENWEATHERMAP_API_KEY", "64bb9b21cbc324788d23f63855a075fd");
    cmd.arg(MOCK_VALID_ZIP_CODE);

    // Assert the structure of the output, not exact dynamic values
    cmd.assert()
        .success()
        .stdout(predicates::str::is_match(r"^📍[A-Za-z\s]+,\s[A-Z]{2}\sT:\d+F\sHi:\d+F\sLo:\d+F\s.\s.\d+kts\s💧\d+%\sP:\d+\.\d{2}inHg\s\s🌅\d{2}:\d{2}\s🌇\d{2}:\d{2}$").unwrap());
}

#[test]
fn test_qwx_missing_api_key() {
    unsafe { env::remove_var("OPENWEATHERMAP_API_KEY"); } // Ensure unset

    let mut cmd = assert_cmd::Command::cargo_bin("qwx").unwrap();
    cmd.arg(MOCK_VALID_ZIP_CODE);

    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("OPENWEATHERMAP_API_KEY environment variable not set"));
}

#[test]
fn test_qwx_invalid_zip_code_live_api() {

    let mut cmd = assert_cmd::Command::cargo_bin("qwx").unwrap();
    cmd.env("OPENWEATHERMAP_API_KEY", "64bb9b21cbc324788d23f63855a075fd");
    cmd.arg(MOCK_INVALID_ZIP_CODE);

    // Expecting an API error from OpenWeatherMap for an invalid zip code
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Geocoding Error: Latitude not found in geocoding response")
            .or(predicates::str::contains("Geocoding API returned status 400")) // OpenWeatherMap returns 400 for bad zip
            .or(predicates::str::contains("Geocoding API returned status 404 Not Found"))
            .or(predicates::str::contains("Latitude not found in geocoding response")) // Fallback error from data extraction
        );
}
