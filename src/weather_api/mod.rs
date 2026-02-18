pub mod open_meteo;

use std::fmt;
use super::model::WeatherReport;

/// Enum to select the weather API provider.
pub enum WeatherApiProvider {
    OpenWeatherMap,
    OpenMeteo,
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

pub async fn get_weather(
    zip_code: &str,
    provider: WeatherApiProvider,
) -> Result<WeatherReport, WeatherApiError> {
    match provider {
        WeatherApiProvider::OpenWeatherMap => {
            Err(WeatherApiError::ApiError("OpenWeatherMap API is currently not implemented.".to_string()))
        },
        WeatherApiProvider::OpenMeteo => {
            // Use Open-Meteo's own geocoding by passing the search term directly
            open_meteo::get_current_weather_report(zip_code).await.map_err(Into::into)
        }
    }
}
