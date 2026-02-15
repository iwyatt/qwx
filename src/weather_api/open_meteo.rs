use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use open_meteo_rs::{geocoding, forecast}; // Import forecast module for Options
use open_meteo_rs::forecast::{ForecastResult, ForecastResultHourly}; // Only import ForecastResult and ForecastResultHourly
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::model::{WeatherCondition, WeatherReport};

// --- Open-Meteo API Models (simplified for mapping to existing model.rs) ---

// This struct will hold the relevant current weather data from Open-Meteo
// and will be used to convert to our generic WeatherReport.
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenMeteoWeatherResponse {
    pub forecast_data: ForecastResult, // Holds the raw ForecastResult
}

// --- Open-Meteo Client ---

pub async fn get_current_weather_report(search_term: &str) -> Result<WeatherReport> {
    let client = open_meteo_rs::Client::new();
    let geocoding_opts = geocoding::Options {
        name: Some(String::from(search_term)),
        count: Some(1),
        apikey: None,
        language: None,
    };

    let geocoding_response = client.geocoding(geocoding_opts)
        .await
        .map_err(|e| anyhow!("Open-Meteo geocoding API error: {}", e))?; // Convert error to anyhow

    let first_location = geocoding_response.results
        .and_then(|mut results| results.drain(..).next()) // Take the first result, consuming the vector
        .ok_or_else(|| anyhow!("No location found for search term: {}", search_term))?;

    let lat = first_location.latitude.unwrap_or_else(|| { // Unwrap Option<f64>
        panic!("Location latitude is missing for search term: {}", search_term);
    });
    let lng = first_location.longitude.unwrap_or_else(|| { // Unwrap Option<f64>
        panic!("Location longitude is missing for search term: {}", search_term);
    });
    let country_code = first_location.country_code.unwrap_or_else(|| { // Unwrap Option<String>
        panic!("Location country code is missing for search term: {}", search_term);
    });
    let location_name = first_location.admin4
        .or(first_location.admin3)
        .or(first_location.admin2)
        .or(first_location.admin1)
        .or(first_location.name) // Fallback to primary name if no admin level is found
        .unwrap_or_else(|| "Unknown Location".to_string());


    // Now fetch the weather forecast using the obtained lat and lng
    let mut opts = forecast::Options::default(); // Use forecast::Options
    opts.location = open_meteo_rs::Location { // This Location is actually the geo-coordinates struct
        lat: lat, // Use the unwrapped lat
        lng: lng, // Use the unwrapped lng
    };
    opts.temperature_unit = Some(open_meteo_rs::forecast::TemperatureUnit::Fahrenheit);
    opts.wind_speed_unit = Some(open_meteo_rs::forecast::WindSpeedUnit::Kn);
    opts.precipitation_unit = Some(open_meteo_rs::forecast::PrecipitationUnit::Inches);

    let start_date = chrono::Utc::now()
        .naive_local()
        .date();
    opts.start_date = Some(start_date);
    opts.end_date = Some(start_date + chrono::Duration::days(1));

    // Request current weather and necessary hourly data for min/max temperature
    opts.current.push("temperature_2m".into());
    opts.current.push("precipitation".into());
    opts.current.push("weather_code".into());
    opts.current.push("wind_speed_10m".into());
    opts.current.push("wind_direction_10m".into());
    opts.hourly.push("temperature_2m".into()); // For min/max calculation

    let forecast_response = client.forecast(opts)
        .await
        .map_err(|e| anyhow!("Open-Meteo forecast API error: {}", e))?; // Convert error to anyhow

    // Convert the Forecast response to our internal OpenMeteoWeatherResponse
    let om_response = OpenMeteoWeatherResponse {
        forecast_data: forecast_response,
    };

    let current_weather_data = om_response.forecast_data.current
        .ok_or_else(|| anyhow!("No current weather data found in Open-Meteo response"))?;

    let datetime = current_weather_data.datetime.and_utc(); // Convert NaiveDateTime to DateTime<Utc>

    let weather_code_val = current_weather_data.values.get("weather_code")
        .ok_or_else(|| anyhow!("Failed to get weather_code from values"))?;
    let weather_code: u8 = weather_code_val.value.as_f64()
        .ok_or_else(|| anyhow!("Failed to convert weather_code to f64"))? as u8;
    let weather_condition = WeatherCondition::from_wmo_code(weather_code);

    let temperature_val = current_weather_data.values.get("temperature")
        .ok_or_else(|| anyhow!("Failed to get temperature from values"))?;
    let temperature: f64 = temperature_val.value.as_f64()
        .ok_or_else(|| anyhow!("Failed to convert temperature to f64"))?;

    let wind_speed_val = current_weather_data.values.get("wind_speed")
        .ok_or_else(|| anyhow!("Failed to get wind_speed from values"))?;
    let wind_speed: f64 = wind_speed_val.value.as_f64()
        .ok_or_else(|| anyhow!("Failed to convert wind_speed to f64"))?;

    let wind_direction_val = current_weather_data.values.get("wind_direction")
        .ok_or_else(|| anyhow!("Failed to get wind_direction from values"))?;
    let wind_direction: f64 = wind_direction_val.value.as_f64()
        .ok_or_else(|| anyhow!("Failed to convert wind_direction to f64"))?;

    let mut weather_report = WeatherReport {
        city_name: Some(location_name),
        country: Some(country_code),
        temperature: temperature,
        feels_like: temperature, // Feels like defaults to temperature if not available
        temp_min: None, // Will be set from hourly data if available
        temp_max: None, // Will be set from hourly data if available
        pressure: None, // Not directly available in CurrentWeather
        humidity: None, // Not directly available in CurrentWeather
        wind_speed: wind_speed,
        wind_deg: Some(wind_direction as u16),
        sunrise: None, // Open-Meteo provides daily sunrise/sunset, not in current_weather, handle separately if needed
        sunset: None, // Open-Meteo provides daily sunrise/sunset, not in current_weather, handle separately if needed
        weather_condition,
        datetime,
        timezone_offset: None, // Can be derived from om_response.timezone if needed
        latitude: lat,
        longitude: lng,
    };

    // Manually calculate min/max temperature from hourly data if available
    if let Some(hourly_data) = om_response.forecast_data.hourly {
        let temps: Vec<f64> = hourly_data.into_iter()
            .filter_map(|item| {
                item.values.get("temperature_2m")
                    .and_then(|val| val.value.as_f64())
            })
            .collect();

        if !temps.is_empty() {
            weather_report.temp_min = Some(temps.iter().copied().fold(f64::INFINITY, f64::min));
            weather_report.temp_max = Some(temps.iter().copied().fold(f64::NEG_INFINITY, f64::max));
        }
    }

    Ok(weather_report)
}


