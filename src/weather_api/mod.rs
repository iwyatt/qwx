use std::{env, fmt};
use reqwest::Url;
use super::model::CurrentWeather;

pub mod owm_models;

// OpenWeatherMap API base URLs
const GEOCODING_API_BASE_URL: &str = "http://api.openweathermap.org/geo/1.0/zip";
const CURRENT_WEATHER_API_BASE_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

/// Custom error type for the weather_api module.
#[derive(Debug)]
pub enum WeatherApiError {
    EnvironmentVarError(String),
    UrlParseError(String),
    NetworkError(reqwest::Error),
    GeocodingError(String),
    ApiError(String), // For errors returned by the API itself
    JsonParseError(serde_json::Error),
}

impl fmt::Display for WeatherApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeatherApiError::EnvironmentVarError(msg) => write!(f, "Environment Variable Error: {}", msg),
            WeatherApiError::UrlParseError(msg) => write!(f, "URL Parsing Error: {}", msg),
            WeatherApiError::NetworkError(err) => write!(f, "Network Error: {}", err),
            WeatherApiError::GeocodingError(msg) => write!(f, "Geocoding Error: {}", msg),
            WeatherApiError::ApiError(msg) => write!(f, "API Error: {}", msg),
            WeatherApiError::JsonParseError(err) => write!(f, "JSON Parsing Error: {}", err),
        }
    }
}

impl std::error::Error for WeatherApiError {}

pub async fn get_lat_lon_from_zip(
    zip_code: &str,
    api_key: &str,
) -> Result<(f64, f64, String, String), WeatherApiError> {
    let url = Url::parse_with_params(
        GEOCODING_API_BASE_URL,
        &[("zip", &format!("{},us", zip_code)), ("appid", &String::from(api_key))],
    )
    .map_err(|e| WeatherApiError::UrlParseError(format!("Failed to parse geocoding URL: {}", e)))?;

    let response = reqwest::get(url)
        .await
        .map_err(WeatherApiError::NetworkError)?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_else(|_| "N/A".to_string());
        return Err(WeatherApiError::ApiError(format!(
            "Geocoding API returned status {}: {}",
            status, body
        )));
    }

    let json_response = response
        .json::<owm_models::OwmGeocodingResponse>()
        .await
        .map_err(WeatherApiError::NetworkError)?;

    Ok((
        json_response.lat,
        json_response.lon,
        json_response.name,
        json_response.country,
    ))
}
/// Constructs the OpenWeatherMap Current Weather API URL.
/// Requires a zip code for location and retrieves the API key from environment variables.
pub async fn fetch_and_parse_current_weather(zip_code: &str) -> Result<CurrentWeather, WeatherApiError> {
    let api_key = env::var("OPENWEATHERMAP_API_KEY")
        .map_err(|e| WeatherApiError::EnvironmentVarError(format!("OPENWEATHERMAP_API_KEY environment variable not set: {}", e)))?;

    let (lat, lon, city_name, country) = get_lat_lon_from_zip(zip_code, &api_key).await?;

    let url = Url::parse_with_params(
        CURRENT_WEATHER_API_BASE_URL,
        &[
            ("lat", lat.to_string().as_str()),
            ("lon", lon.to_string().as_str()),
            ("appid", api_key.as_str()),
            ("units", "imperial"), // Specify imperial units as per PRD
        ],
    )
    .map_err(|e| WeatherApiError::UrlParseError(format!("Failed to parse Current Weather API URL: {}", e)))?;

    let response = reqwest::get(url)
        .await
        .map_err(WeatherApiError::NetworkError)?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_else(|_| "N/A".to_string());
        return Err(WeatherApiError::ApiError(format!(
            "Current Weather API returned status {}: {}",
            status, body
        )));
    }

    let json_response = response
        .json::<serde_json::Value>()
        .await
        .map_err(WeatherApiError::NetworkError)?;

    let owm_current_response: owm_models::OwmCurrentWeatherResponse = serde_json::from_value(json_response)
        .map_err(WeatherApiError::JsonParseError)?;

    Ok(CurrentWeather::new(
        city_name,
        country,
        &owm_current_response,
    ))
}
