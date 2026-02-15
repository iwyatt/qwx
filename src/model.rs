use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, TimeZone};
use crate::weather_api::owm_models;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum WeatherCondition {
    Clear,
    Clouds,
    Rain,
    Thunderstorm,
    Snow,
    Mist,
    Fog,
    Haze,
    Smoke,
    Dust,
    Sand,
    Ash,
    Squall,
    Tornado,
    Drizzle,
    FreezingDrizzle,
    FreezingRain,
    Unknown,
}

impl WeatherCondition {
    pub fn from_main(main: &str) -> Self {
        match main {
            "Clear" => WeatherCondition::Clear,
            "Clouds" => WeatherCondition::Clouds,
            "Rain" => WeatherCondition::Rain,
            "Thunderstorm" => WeatherCondition::Thunderstorm,
            "Snow" => WeatherCondition::Snow,
            "Mist" => WeatherCondition::Mist,
            "Fog" => WeatherCondition::Fog,
            "Haze" => WeatherCondition::Haze,
            "Smoke" => WeatherCondition::Smoke,
            "Dust" => WeatherCondition::Dust,
            "Sand" => WeatherCondition::Sand,
            "Ash" => WeatherCondition::Ash,
            "Squall" => WeatherCondition::Squall,
            "Tornado" => WeatherCondition::Tornado,
            "Drizzle" => WeatherCondition::Drizzle,
            _ => WeatherCondition::Unknown,
        }
    }

    // A simplified mapping from WMO Weather Interpretation Codes to our WeatherCondition
    // This is a basic mapping and can be made more sophisticated.
    pub fn from_wmo_code(code: u8) -> Self {
        match code {
            0 => WeatherCondition::Clear, // Clear sky
            1 | 2 | 3 => WeatherCondition::Clouds, // Mainly clear, partly cloudy, overcast
            45 | 48 => WeatherCondition::Fog, // Fog and depositing rime fog
            51 | 53 | 55 => WeatherCondition::Drizzle, // Drizzle: Light, moderate, and dense intensity
            56 | 57 => WeatherCondition::FreezingDrizzle, // Freezing Drizzle: Light and dense intensity
            61 | 63 | 65 => WeatherCondition::Rain, // Rain: Slight, moderate and heavy intensity
            66 | 67 => WeatherCondition::FreezingRain, // Freezing Rain: Light and heavy intensity
            71 | 73 | 75 => WeatherCondition::Snow, // Snow fall: Slight, moderate, and heavy intensity
            77 => WeatherCondition::Snow, // Snow grains
            80 | 81 | 82 => WeatherCondition::Rain, // Rain showers: Slight, moderate, and violent
            85 | 86 => WeatherCondition::Snow, // Snow showers slight and heavy
            95 => WeatherCondition::Thunderstorm, // Thunderstorm: Slight or moderate
            96 | 99 => WeatherCondition::Thunderstorm, // Thunderstorm with slight and heavy hail
            _ => WeatherCondition::Unknown,
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            WeatherCondition::Clear => "☀️",
            WeatherCondition::Clouds => "☁️",
            WeatherCondition::Rain | WeatherCondition::Drizzle | WeatherCondition::FreezingDrizzle | WeatherCondition::FreezingRain => "🌧️",
            WeatherCondition::Thunderstorm => "⛈️",
            WeatherCondition::Snow => "🌨️",
            WeatherCondition::Mist | WeatherCondition::Fog | WeatherCondition::Haze | WeatherCondition::Smoke | WeatherCondition::Dust | WeatherCondition::Sand | WeatherCondition::Ash => "🌫️", // Using a single emoji for atmospheric conditions
            WeatherCondition::Squall | WeatherCondition::Tornado => "🌪️",
            WeatherCondition::Unknown => "❓",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherReport {
    pub city_name: Option<String>,
    pub country: Option<String>,
    pub temperature: f64,
    pub feels_like: f64,
    pub temp_min: Option<f64>,
    pub temp_max: Option<f64>,
    pub pressure: Option<u16>, // hPa
    pub humidity: Option<u8>,
    pub wind_speed: f64, // km/h (standardized for display)
    pub wind_deg: Option<u16>, // degrees (0-360)
    pub sunrise: Option<DateTime<Utc>>,
    pub sunset: Option<DateTime<Utc>>,
    pub weather_condition: WeatherCondition,
    pub datetime: DateTime<Utc>,
    pub timezone_offset: Option<i32>, // Offset in seconds from UTC
    pub latitude: f64,
    pub longitude: f64,
}

impl WeatherReport {
    pub fn from_owm(
        city_name: &str,
        country: &str,
        owm_response: &owm_models::OwmCurrentWeatherResponse,
    ) -> Self {
        WeatherReport {
            city_name: Some(city_name.to_string()),
            country: Some(country.to_string()),
            temperature: owm_response.main.temp,
            feels_like: owm_response.main.feels_like,
            temp_min: Some(owm_response.main.temp_min),
            temp_max: Some(owm_response.main.temp_max),
            pressure: Some(owm_response.main.pressure),
            humidity: Some(owm_response.main.humidity),
            wind_speed: owm_response.wind.speed * 3.6, // Convert m/s to km/h for consistency
            wind_deg: Some(owm_response.wind.deg),
            sunrise: Utc.timestamp_opt(owm_response.sys.sunrise, 0).single(),
            sunset: Utc.timestamp_opt(owm_response.sys.sunset, 0).single(),
            weather_condition: owm_response
                .weather
                .get(0)
                .map(|owm_weather| WeatherCondition::from_main(&owm_weather.main))
                .unwrap_or(WeatherCondition::Unknown),
            datetime: Utc.timestamp_opt(owm_response.dt, 0).single().unwrap_or_else(Utc::now),
            timezone_offset: Some(owm_response.timezone),
            latitude: 0.0, // OWM Current Weather response does not directly include lat/lon here
            longitude: 0.0, // OWM Current Weather response does not directly include lat/lon here
        }
    }
}

