use anyhow::{Result, anyhow};
use open_meteo_rs::{geocoding, forecast}; // Import forecast module for Options
use open_meteo_rs::forecast::ForecastResult; // Only import ForecastResult and ForecastResultHourly
use serde::{Deserialize, Serialize};
use crate::model::{WeatherCondition, WeatherReport};
use chrono::{Utc, TimeZone, Offset};
use chrono_tz::Tz;

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
    let admin1_cloned = first_location.admin1.clone();

    let location_name = first_location.name
        .or(admin1_cloned.clone())
        .or(first_location.admin2)
        .or(first_location.admin3)
        .or(first_location.admin4)
        .unwrap_or_else(|| "Unknown Location".to_string());
    
    let state = admin1_cloned;
    
    
    // Extract timezone from geocoding response
    let timezone_offset_seconds = if let Some(tz_str) = first_location.timezone {
        tz_str.parse::<Tz>()
            .ok()
            .map(|tz| tz.offset_from_utc_datetime(&Utc::now().naive_utc()).fix().local_minus_utc())
    } else {
        None
    };


    // Now fetch the weather forecast using the obtained lat and lng
    let mut opts = forecast::Options::default(); // Use forecast::Options
    opts.location = open_meteo_rs::Location { // This Location is actually the geo-coordinates struct
        lat: lat, // Use the unwrapped lat
        lng: lng, // Use the unwrapped lng
    };
    opts.temperature_unit = Some(open_meteo_rs::forecast::TemperatureUnit::Fahrenheit);
    opts.wind_speed_unit = Some(open_meteo_rs::forecast::WindSpeedUnit::Kn);
    opts.precipitation_unit = Some(open_meteo_rs::forecast::PrecipitationUnit::Inches);



    opts.current.push("temperature_2m".into());
    opts.current.push("dew_point_2m".into());
    opts.current.push("weather_code".into());
    opts.current.push("wind_speed_10m".into());
    opts.current.push("wind_direction_10m".into());
    opts.current.push("relative_humidity_2m".into());
    opts.current.push("precipitation_probability".into());
    opts.current.push("surface_pressure".into()); // Re-add surface pressure


    opts.hourly.push("temperature_2m".into());
    opts.hourly.push("weather_code".into());
    opts.hourly.push("precipitation_probability".into());
    opts.hourly.push("wind_speed_10m".into());
    opts.hourly.push("wind_direction_10m".into());
    opts.hourly.push("dew_point_2m".into());
    opts.hourly.push("apparent_temperature".into());
    opts.hourly.push("relative_humidity_2m".into());
    opts.hourly.push("surface_pressure".into());
    opts.daily.push("sunrise".into());
    opts.daily.push("sunset".into());
    opts.daily.push("weather_code".into());
    opts.daily.push("temperature_2m_max".into());
    opts.daily.push("temperature_2m_min".into());
    opts.daily.push("precipitation_sum".into());
    opts.daily.push("precipitation_probability_max".into());
    opts.forecast_days = Some(7);

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

    let temperature_val = current_weather_data.values.get("temperature_2m")
        .ok_or_else(|| anyhow!("Failed to get temperature_2m from values"))?;
    let temperature: f64 = temperature_val.value.as_f64()
        .ok_or_else(|| anyhow!("Failed to convert temperature to f64"))?;

    let dew_point_val = current_weather_data.values.get("dew_point_2m")
        .ok_or_else(|| anyhow!("Failed to get dew_point_2m from values"))?;
    let dew_point: f64 = dew_point_val.value.as_f64()
        .ok_or_else(|| anyhow!("Failed to convert dew_point to f64"))?;

    let wind_speed_val = current_weather_data.values.get("wind_speed_10m")
        .ok_or_else(|| anyhow!("Failed to convert wind_speed to f64"))?;
    let wind_speed: f64 = wind_speed_val.value.as_f64()
        .ok_or_else(|| anyhow!("Failed to convert wind_speed to f64"))?;

    let wind_direction_val = current_weather_data.values.get("wind_direction_10m")
        .ok_or_else(|| anyhow!("Failed to convert wind_direction_10m to f64"))?;
    let wind_direction: f64 = wind_direction_val.value.as_f64()
        .ok_or_else(|| anyhow!("Failed to convert wind_direction_10m to f64"))?;

    let humidity_val = current_weather_data.values.get("relative_humidity_2m")
        .ok_or_else(|| anyhow!("Failed to get relative_humidity_2m from values"))?;
    let humidity: u8 = humidity_val.value.as_f64()
        .ok_or_else(|| anyhow!("Failed to convert relative_humidity_2m to f64"))? as u8;

    let pressure_val = current_weather_data.values.get("surface_pressure")
        .ok_or_else(|| anyhow!("Failed to get surface_pressure from values"))?;
    let pressure: u16 = pressure_val.value.as_f64()
        .ok_or_else(|| anyhow!("Failed to convert surface_pressure to f64"))? as u16;

    let current_precipitation_chance: Option<u8> = current_weather_data.values.get("precipitation_probability")
        .and_then(|val| val.value.as_f64())
        .map(|v| v as u8);

    let mut weather_report = WeatherReport {
        city_name: Some(location_name),
        country: Some(country_code),
        temperature: temperature,
        dew_point: Some(dew_point),
        feels_like: temperature, // Feels like defaults to temperature if not available
        temp_min: None, // Will be set from hourly data if available
        temp_max: None, // Will be set from hourly data if available
        pressure: Some(pressure), // Now available
        humidity: Some(humidity), // Now available
        current_precipitation_chance,
        wind_speed: wind_speed,
        wind_deg: Some(wind_direction as u16),
        sunrise: None, // Open-Meteo provides daily sunrise/sunset, not in current_weather, handle separately if needed
        sunset: None, // Open-Meteo provides daily sunrise/sunset, not in current_weather, handle separately if needed
        weather_condition,
        datetime,
        timezone_offset: timezone_offset_seconds, // Now derived from geocoding_response
        latitude: lat,
        longitude: lng,
        daily_forecast: Vec::new(),
        hourly_forecast: Vec::new(),
        metar: None,
        taf: None,
        state,
    };

    // Manually calculate min/max temperature from hourly data if available
    if let Some(hourly_data) = &om_response.forecast_data.hourly {
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

    if let Some(hourly_data_entries) = om_response.forecast_data.hourly {
        let mut hourly_entries = Vec::new();
        for hourly_item in hourly_data_entries.into_iter() {
            let hourly_values = &hourly_item.values;

            let time = hourly_item.datetime.and_utc();

            let temperature = hourly_values.get("temperature_2m")
                .and_then(|val| val.value.as_f64())
                .ok_or_else(|| anyhow!("Failed to get hourly temperature"))?;

            let weather_code = hourly_values.get("weather_code")
                .and_then(|val| val.value.as_f64())
                .map(|v| v as u8)
                .ok_or_else(|| anyhow!("Failed to get hourly weather_code"))?;
            let weather_condition = WeatherCondition::from_wmo_code(weather_code);

            let precipitation_chance = hourly_values.get("precipitation_probability")
                .and_then(|val| val.value.as_f64())
                .map(|v| v as u8);

            let wind_speed = hourly_values.get("wind_speed_10m")
                .and_then(|val| val.value.as_f64())
                .ok_or_else(|| anyhow!("Failed to get hourly wind_speed"))?;

            let wind_deg = hourly_values.get("wind_direction_10m")
                .and_then(|val| val.value.as_f64())
                .map(|v| v as u16);

            let dew_point = hourly_values.get("dew_point_2m")
                .and_then(|val| val.value.as_f64());

            let feels_like = hourly_values.get("apparent_temperature")
                .and_then(|val| val.value.as_f64());

            let humidity = hourly_values.get("relative_humidity_2m")
                .and_then(|val| val.value.as_f64())
                .map(|v| v as u8);

            let pressure = hourly_values.get("surface_pressure")
                .and_then(|val| val.value.as_f64())
                .map(|v| v as u16);

            hourly_entries.push(crate::model::HourlyForecastEntry {
                time,
                temperature,
                weather_condition,
                precipitation_chance,
                wind_speed,
                wind_deg,
                dew_point,
                feels_like,
                humidity,
                pressure,
            });
        }
        weather_report.hourly_forecast = hourly_entries;
    }

    // Handle daily data for sunrise/sunset and 6-day forecast
    if let Some(daily_forecasts_vec) = om_response.forecast_data.daily {
        let mut daily_entries = Vec::new();
        for daily_item in daily_forecasts_vec.into_iter() {
            let daily_values = &daily_item.values;

            let weather_code_val = daily_values.get("weather_code")
                .and_then(|val| val.value.as_f64())
                .map(|v| v as u8);
            let weather_condition = weather_code_val.map(WeatherCondition::from_wmo_code)
                .unwrap_or(WeatherCondition::Unknown);

            let temp_max = daily_values.get("temperature_2m_max")
                .and_then(|val| val.value.as_f64());
            let temp_min = daily_values.get("temperature_2m_min")
                .and_then(|val| val.value.as_f64());

            let precipitation_chance = daily_values.get("precipitation_probability_max")
                .and_then(|val| val.value.as_f64())
                .map(|v| v as u8);
            
            // For sunrise/sunset, only take the first day's values if available
            if weather_report.sunrise.is_none() {
                let sunrise_val = daily_values.get("sunrise")
                    .and_then(|item| item.value.as_i64());
                if let Some(sr_ts) = sunrise_val {
                    if let Some(sunrise_dt) = Utc.timestamp_opt(sr_ts, 0).single() {
                        weather_report.sunrise = Some(sunrise_dt);
                    }
                }
            }
            if weather_report.sunset.is_none() {
                let sunset_val = daily_values.get("sunset")
                    .and_then(|item| item.value.as_i64());
                if let Some(ss_ts) = sunset_val {
                    if let Some(sunset_dt) = Utc.timestamp_opt(ss_ts, 0).single() {
                        weather_report.sunset = Some(sunset_dt);
                    }
                }
            }

            if let (Some(temp_max), Some(temp_min)) = (temp_max, temp_min) {
                daily_entries.push(crate::model::DailyForecastEntry {
                    date: daily_item.date,
                    weather_condition,
                    temp_max,
                    temp_min,
                    precipitation_chance,
                });
            }
        }
        weather_report.daily_forecast = daily_entries;
    }

    Ok(weather_report)
}


