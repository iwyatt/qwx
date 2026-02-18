use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
pub struct DailyForecastEntry {
    pub date: chrono::NaiveDate,
    pub weather_condition: WeatherCondition,
    pub temp_max: f64,
    pub temp_min: f64,
    pub precipitation_chance: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherReport {
    pub city_name: Option<String>,
    pub state: Option<String>,
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
    pub daily_forecast: Vec<DailyForecastEntry>,
}



