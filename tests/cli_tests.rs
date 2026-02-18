use assert_cmd::Command;
use predicates::str::contains;

const MOCK_VALID_ZIP_CODE: &str = "90210"; 
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
