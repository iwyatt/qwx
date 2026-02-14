use serde::{Deserialize, Serialize};
use crate::model::WeatherCondition;

// Intermediate structs to parse OpenWeatherMap API response
// These mirror the API's JSON structure more closely

#[derive(Debug, Serialize, Deserialize)]
pub struct OwmGeocodingResponse {
    pub zip: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwmWeather {
    pub id: u16,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwmMain {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: u16,
    pub humidity: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwmWind {
    pub speed: f64,
    pub deg: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwmSys {
    #[serde(rename = "type")]
    pub type_field: Option<u16>, // 'type' is a reserved keyword, use type_field
    pub id: Option<u32>,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwmCurrentWeatherResponse {
    pub weather: Vec<OwmWeather>,
    pub main: OwmMain,
    pub wind: OwmWind,
    pub sys: OwmSys,
    pub name: String,
    pub dt: i64,
    pub timezone: i32,
}

// --- Conversion Implementations ---

impl From<&OwmWeather> for WeatherCondition {
    fn from(owm_weather: &OwmWeather) -> Self {
        WeatherCondition::from_main(&owm_weather.main)
    }
}
