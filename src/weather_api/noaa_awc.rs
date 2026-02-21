use anyhow::{Result, anyhow};
use crate::model::{WeatherReport, WeatherCondition, MetarReport, TafReport};
use chrono::Utc;

pub async fn get_aviation_weather_report(icao: &str) -> Result<WeatherReport> {
    let client = reqwest::Client::new();
    
    // Fetch METAR
    let metar_url = format!("https://aviationweather.gov/api/data/metar?ids={}&format=raw", icao);
    let metar_raw = client.get(metar_url).send().await?.text().await?;
    
    if metar_raw.trim().is_empty() {
        return Err(anyhow!("No METAR found for station: {}", icao));
    }

    // Fetch TAF
    let taf_url = format!("https://aviationweather.gov/api/data/taf?ids={}&format=raw", icao);
    let taf_raw = client.get(taf_url).send().await?.text().await?.trim().to_string();

    let mut taf_report = None;
    if !taf_raw.is_empty() {
        let lines = taf_raw.split("  ").map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        taf_report = Some(TafReport {
            raw: taf_raw,
            station_id: icao.to_uppercase(),
            lines,
        });
    }

    // For now, we return a hybrid report. 
    // The main fields will be largely empty as the display logic will prioritize the METAR/TAF sections.
    Ok(WeatherReport {
        city_name: Some(icao.to_uppercase()),
        country: Some("".to_string()),
        temperature: 0.0, // We'll parse these if needed, but the priority is the raw string
        dew_point: None,
        feels_like: 0.0,
        state: None,
        temp_min: None,
        temp_max: None,
        pressure: None,
        humidity: None,
        wind_speed: 0.0,
        wind_deg: None,
        sunrise: None,
        sunset: None,
        weather_condition: WeatherCondition::Unknown,
        datetime: Utc::now(),
        timezone_offset: None,
        latitude: 0.0,
        longitude: 0.0,
        daily_forecast: Vec::new(),
        metar: Some(MetarReport {
            raw: metar_raw.trim().to_string(),
            station_id: icao.to_uppercase(),
            observation_time: Utc::now(),
        }),
        taf: taf_report,
    })
}
