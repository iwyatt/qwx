pub mod open_meteo;
pub mod noaa_awc;

use std::fmt;
use super::model::WeatherReport;
use regex::Regex;
use reqwest;
use serde_json::Value;

/// Enum to select the weather API provider.
pub enum WeatherApiProvider {
    OpenMeteo,
    NoaaAwc,
}

/// Custom error type for the weather_api module.
#[derive(Debug)]
pub enum WeatherApiError {
    ApiError(String), // For errors returned by the API itself
    Other(anyhow::Error),
}

impl fmt::Display for WeatherApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeatherApiError::ApiError(msg) => write!(f, "API Error: {}", msg),
            WeatherApiError::Other(err) => write!(f, "Other Error: {}", err),
        }
    }
}

impl std::error::Error for WeatherApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WeatherApiError::Other(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl From<anyhow::Error> for WeatherApiError {
    fn from(err: anyhow::Error) -> Self {
        WeatherApiError::Other(err)
    }
}

pub async fn get_location_from_ip() -> Result<String, anyhow::Error> {
    let client = reqwest::Client::new();
    let res = client.get("http://ip-api.com/json/?fields=zip").send().await?
        .error_for_status()?;
    
    let json: Value = res.json().await?;
    
    if json["status"] == "fail" {
        anyhow::bail!("Failed to get location from IP: {}", json["message"].as_str().unwrap_or("Unknown error"));
    }

    if let Some(zip) = json["zip"].as_str() {
        Ok(zip.to_string())
    } else {
        anyhow::bail!("Could not find zip code in IP-API response.");
    }
}

pub async fn get_weather(
    location: &str,
    provider: WeatherApiProvider,
) -> Result<WeatherReport, WeatherApiError> {
    let zip_regex = Regex::new(r"^\d{5}$").unwrap();
    let aviation_regex = Regex::new(r"^[a-zA-Z0-9]{3,4}$").unwrap();

    let actual_provider = match provider {
        WeatherApiProvider::OpenMeteo if aviation_regex.is_match(location) && !zip_regex.is_match(location) => {
            WeatherApiProvider::NoaaAwc
        }
        p => p,
    };

    let mut attempts = 0;
    let max_attempts = 3;

    loop {
        attempts += 1;
        let result = match actual_provider {
            WeatherApiProvider::OpenMeteo => {
                open_meteo::get_current_weather_report(location).await.map_err(Into::into)
            },
            WeatherApiProvider::NoaaAwc => {
                noaa_awc::get_aviation_weather_report(location).await.map_err(Into::into)
            }
        };

        match result {
            Ok(report) => return Ok(report),
            Err(e) if attempts < max_attempts => {
                // Check if it's an error that should NOT be retried (e.g. location not found)
                let err_msg = format!("{}", e);
                if err_msg.contains("No location found") || err_msg.contains("not implemented") {
                    return Err(e);
                }
                
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
}
