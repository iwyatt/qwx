use assert_cmd::Command;
use predicates::str::contains;

const MOCK_INVALID_ZIP_CODE: &str = "00000"; 

#[test]
fn test_qwx_invalid_zip_code_live_api() {
    let mut cmd = Command::cargo_bin("qwx").unwrap();
    cmd.arg(MOCK_INVALID_ZIP_CODE);

    // Expecting an API error from Open-Meteo for an invalid zip code
    cmd.assert()
        .failure()
        .stderr(contains("Error: Other(No location found for search term: 00000)"));
}

#[test]
fn test_qwx_default_no_forecast_live_api() {
    let mut cmd = Command::cargo_bin("qwx").unwrap();
    cmd.arg("90210");

    let assert = cmd.assert().success();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    // Default should only have one line
    assert_eq!(output.lines().count(), 1);
}

#[test]
fn test_qwx_with_forecast_live_api() {
    let mut cmd = Command::cargo_bin("qwx").unwrap();
    cmd.arg("90210").arg("--forecast");

    let assert = cmd.assert().success();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    
    // With forecast should have multiple lines
    assert!(output.lines().count() > 1);
}
