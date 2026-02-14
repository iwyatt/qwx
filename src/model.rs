use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, TimeZone};

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

    pub fn emoji(&self) -> &str {
        match self {
            WeatherCondition::Clear => "☀️",
            WeatherCondition::Clouds => "☁️",
            WeatherCondition::Rain | WeatherCondition::Drizzle => "🌧️",
            WeatherCondition::Thunderstorm => "⛈️",
            WeatherCondition::Snow => "🌨️",
            WeatherCondition::Mist | WeatherCondition::Fog | WeatherCondition::Haze | WeatherCondition::Smoke | WeatherCondition::Dust | WeatherCondition::Sand | WeatherCondition::Ash => "🌫️", // Using a single emoji for atmospheric conditions
            WeatherCondition::Squall | WeatherCondition::Tornado => "🌪️",
            WeatherCondition::Unknown => "❓",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub city_name: String,
    pub country: String,
    pub temperature: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub humidity: u8,
    pub condition: WeatherCondition,
    pub wind_speed: f64, // knots
    pub wind_deg: i32, // degrees (0-360)
    pub sunrise: DateTime<Utc>,
    pub sunset: DateTime<Utc>,
    pub timezone_offset_seconds: i32,
    pub pressure_hpa: u16,
}

impl CurrentWeather {
    pub fn new(
        city_name: String,
        country: String,
        owm_response: &crate::weather_api::owm_models::OwmCurrentWeatherResponse,
    ) -> Self {
        CurrentWeather {
            city_name,
            country,
            temperature: owm_response.main.temp,
            feels_like: owm_response.main.feels_like,
            temp_min: owm_response.main.temp_min,
            temp_max: owm_response.main.temp_max,
            humidity: owm_response.main.humidity,
            condition: owm_response
                .weather
                .get(0)
                .map(WeatherCondition::from)
                .unwrap_or(WeatherCondition::Unknown),
            wind_speed: owm_response.wind.speed,
            wind_deg: owm_response.wind.deg as i32,
            sunrise: Utc
                .timestamp_opt(owm_response.sys.sunrise, 0)
                .single()
                .unwrap(),
            sunset: Utc
                .timestamp_opt(owm_response.sys.sunset, 0)
                .single()
                .unwrap(),
            timezone_offset_seconds: owm_response.timezone,
            pressure_hpa: owm_response.main.pressure,
        }
    }
}
