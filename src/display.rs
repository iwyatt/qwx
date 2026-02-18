use crate::model::WeatherReport;
use chrono::FixedOffset;

// Helper function to get wind direction emoji
pub fn get_wind_direction_emoji(degrees: u16) -> &'static str {
    match degrees {
        // N
        349..=360 | 0..=11 => "⬆️",
        // NNE
        12..=33 => "↗️",
        // NE
        34..=56 => "↗️",
        // ENE
        57..=78 => "↗️",
        // E
        79..=101 => "➡️",
        // ESE
        102..=123 => "↘️",
        // SE
        124..=146 => "↘️",
        // SSE
        147..=168 => "↘️",
        // S
        169..=191 => "⬇️",
        // SSW
        192..=213 => "↙️",
        // SW
        214..=236 => "↙️",
        // WSW
        237..=258 => "↙️",
        // W
        259..=281 => "⬅️",
        // WNW
        282..=303 => "↖️",
        // NW
        304..=326 => "↖️",
        // NNW
        337..=348 => "↖️",
        _ => "❓", // Unknown or invalid
    }
}

pub fn format_weather_report(report: &WeatherReport, show_forecast: bool, _show_hourly: bool) -> String {
    let city_name = report.city_name.as_deref().unwrap_or("N/A");
    let state = report.state.as_deref().unwrap_or("");
    let country = report.country.as_deref().unwrap_or("N/A");

    let location_display = if state.is_empty() {
        format!("{}, {}", city_name, country)
    } else {
        format!("{}, {}, {}", city_name, state, country)
    };
    
    // Convert temperature to Fahrenheit if needed (Open-Meteo returns Celsius)
    let temperature_f = report.temperature.round();
    let _feels_like_f = report.feels_like;

    let (temp_max_f, temp_min_f) = if let Some(today) = report.daily_forecast.first() {
        (Some(today.temp_max.round()), Some(today.temp_min.round()))
    } else {
        (report.temp_max.map(|t| t.round()), report.temp_min.map(|t| t.round()))
    };

    let hilo_display = match (temp_max_f, temp_min_f) {
        (Some(max), Some(min)) => format!(" Hi:{:.0}F Lo:{:.0}F", max, min),
        (Some(max), None) => format!(" Hi:{:.0}F Lo:N/A", max),
        (None, Some(min)) => format!(" Hi:N/A Lo:{:.0}F", min),
        (None, None) => " Hi:N/A Lo:N/A".to_string(),
    };

    // Convert wind speed from km/h to knots
    let wind_speed_knots = report.wind_speed / 1.852;

    let local_timezone = report.timezone_offset.map(|offset| FixedOffset::east_opt(offset).unwrap_or(FixedOffset::east_opt(0).unwrap()))
        .unwrap_or(FixedOffset::east_opt(0).unwrap());


    let sunrise_time = report.sunrise.map(|dt| dt.with_timezone(&local_timezone).format("%H:%M").to_string()).unwrap_or_else(|| "N/A".to_string());
    let sunset_time = report.sunset.map(|dt| dt.with_timezone(&local_timezone).format("%H:%M").to_string()).unwrap_or_else(|| "N/A".to_string());

    let pressure_inhg = report.pressure.map(|p| (p as f64) * 0.02953);

    let pressure_display = match pressure_inhg {
        Some(p) => format!("{:.1}", p), // Shorten to one decimal and remove "inHg"
        None => "N/A".to_string(),
    };


    let current_weather_line = format!(
        "📍{} {temp:.0}F{} {condition_emoji} {wind_emoji}{wind_speed:.0}kts 💧{humidity}% {pressure_display}Hg  🌅{sunrise_time} 🌇{sunset_time}",
        location_display,
        hilo_display,
        temp = temperature_f,
        condition_emoji = report.weather_condition.emoji(),
        wind_emoji = report.wind_deg.map(|deg| get_wind_direction_emoji(deg)).unwrap_or("❓"),
        wind_speed = wind_speed_knots,
        humidity = report.humidity.map(|h| format!("{}", h)).unwrap_or_else(|| "N/A".to_string()),

        sunrise_time = sunrise_time,
        sunset_time = sunset_time
    );

    let mut output = vec![current_weather_line];

    // Format daily forecast if requested
    if show_forecast && !report.daily_forecast.is_empty() {
        let daily_forecast_lines = report.daily_forecast.iter()
            .map(|entry| {
                let day_name = entry.date.format("%a").to_string(); // Abbreviated day name
                let condition_emoji = entry.weather_condition.emoji();
                let precip_chance = entry.precipitation_chance.map(|p| format!("{}%", p)).unwrap_or_else(|| "N/A".to_string());
                format!("{}: Hi {:.0}F Lo {:.0}F {} {}", day_name, entry.temp_max, entry.temp_min, condition_emoji, precip_chance)
            })
            .collect::<Vec<String>>();

        // Group daily forecasts into lines, trying to stay within 80 chars
        let mut current_line = String::new();
        for (i, forecast_str) in daily_forecast_lines.iter().enumerate() {
            if current_line.is_empty() {
                current_line.push_str(forecast_str);
            } else if (current_line.len() + 3 + forecast_str.len()) <= 80 { // 3 for " | "
                current_line.push_str(" | ");
                current_line.push_str(forecast_str);
            } else {
                output.push(current_line.clone());
                current_line = forecast_str.clone();
            }
            if i == daily_forecast_lines.len() - 1 {
                output.push(current_line.clone());
            }
        }
    }

    output.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{WeatherCondition, WeatherReport};
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_get_wind_direction_emoji() {
        assert_eq!(get_wind_direction_emoji(0), "⬆️");   // North
        assert_eq!(get_wind_direction_emoji(10), "⬆️");  // North
        assert_eq!(get_wind_direction_emoji(45), "↗️");  // Northeast
        assert_eq!(get_wind_direction_emoji(90), "➡️");  // East
        assert_eq!(get_wind_direction_emoji(135), "↘️"); // Southeast
        assert_eq!(get_wind_direction_emoji(180), "⬇️"); // South
        assert_eq!(get_wind_direction_emoji(225), "↙️"); // Southwest
        assert_eq!(get_wind_direction_emoji(270), "⬅️"); // West
        assert_eq!(get_wind_direction_emoji(315), "↖️"); // Northwest
        assert_eq!(get_wind_direction_emoji(359), "⬆️"); // North
        assert_eq!(get_wind_direction_emoji(360), "⬆️"); // North
    }

    #[test]
    fn test_format_weather_report() {
        use crate::model::DailyForecastEntry;

        let report = WeatherReport {
            city_name: Some("Testville".to_string()),
            state: Some("TS".to_string()),
            country: Some("US".to_string()),
            temperature: 22.5, // Celsius
            feels_like: 20.0,  // Celsius
            temp_min: Some(18.0), // Celsius
            temp_max: Some(28.0), // Celsius
            pressure: Some(1013), // hPa
            humidity: Some(85),
            weather_condition: WeatherCondition::Clear,
            wind_speed: 18.52, // km/h (10 knots)
            wind_deg: Some(270), // West
            sunrise: Some(Utc.with_ymd_and_hms(2023, 1, 1, 6, 30, 0).unwrap()),
            sunset: Some(Utc.with_ymd_and_hms(2023, 1, 1, 17, 45, 0).unwrap()),
            datetime: Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap(),
            timezone_offset: Some(0), // UTC
            latitude: 0.0,
            longitude: 0.0,
            daily_forecast: vec![
                DailyForecastEntry {
                    date: Utc.with_ymd_and_hms(2023, 1, 2, 0, 0, 0).unwrap().date_naive(),
                    weather_condition: WeatherCondition::Clouds,
                    temp_max: 25.0,
                    temp_min: 15.0,
                    precipitation_chance: Some(30),
                },
                DailyForecastEntry {
                    date: Utc.with_ymd_and_hms(2023, 1, 3, 0, 0, 0).unwrap().date_naive(),
                    weather_condition: WeatherCondition::Rain,
                    temp_max: 20.0,
                    temp_min: 10.0,
                    precipitation_chance: Some(80),
                },
                 DailyForecastEntry {
                    date: Utc.with_ymd_and_hms(2023, 1, 4, 0, 0, 0).unwrap().date_naive(),
                    weather_condition: WeatherCondition::Clear,
                    temp_max: 22.0,
                    temp_min: 12.0,
                    precipitation_chance: Some(10),
                },
                DailyForecastEntry {
                    date: Utc.with_ymd_and_hms(2023, 1, 5, 0, 0, 0).unwrap().date_naive(),
                    weather_condition: WeatherCondition::Snow,
                    temp_max: 5.0,
                    temp_min: -2.0,
                    precipitation_chance: Some(90),
                },
                DailyForecastEntry {
                    date: Utc.with_ymd_and_hms(2023, 1, 6, 0, 0, 0).unwrap().date_naive(),
                    weather_condition: WeatherCondition::Thunderstorm,
                    temp_max: 28.0,
                    temp_min: 18.0,
                    precipitation_chance: Some(70),
                },
                DailyForecastEntry {
                    date: Utc.with_ymd_and_hms(2023, 1, 7, 0, 0, 0).unwrap().date_naive(),
                    weather_condition: WeatherCondition::Mist,
                    temp_max: 15.0,
                    temp_min: 8.0,
                    precipitation_chance: Some(20),
                },
            ],
        };

        let formatted = format_weather_report(&report, true, false);
        let lines: Vec<&str> = formatted.lines().collect();

        // Check expected content of the first line (current weather)
        assert!(lines[0].contains("📍Testville, TS, US 23F Hi:25F Lo:15F ☀️ ⬅️10kts 💧85% 29.9Hg  🌅06:30 🌇17:45"));

        // Check expected content of daily forecast lines
        assert!(lines[1].contains("Mon: Hi 25F Lo 15F ☁️ 30% | Tue: Hi 20F Lo 10F 🌧️ 80%"));
        assert!(lines[2].contains("Wed: Hi 22F Lo 12F ☀️ 10% | Thu: Hi 5F Lo -2F 🌨️ 90%"));
        assert!(lines[3].contains("Fri: Hi 28F Lo 18F ⛈️ 70% | Sat: Hi 15F Lo 8F 🌫️ 20%"));
    }

    #[test]
    fn test_format_weather_report_optional_fields() {
        let report = WeatherReport {
            city_name: None,
            state: None,
            country: None,
            temperature: 20.0, // Celsius
            feels_like: 18.0,  // Celsius
            temp_min: None,
            temp_max: None,
            pressure: None,
            humidity: None,
            weather_condition: WeatherCondition::Unknown,
            wind_speed: 0.0, // km/h
            wind_deg: None,
            sunrise: None,
            sunset: None,
            datetime: Utc::now(),
            timezone_offset: None,
            latitude: 0.0,
            longitude: 0.0,
            daily_forecast: Vec::new(),
        };

        let formatted = format_weather_report(&report, false, false);
        assert_eq!(formatted, "📍N/A, N/A 20F Hi:N/A Lo:N/A ❓ ❓0kts 💧N/A% N/AHg  🌅N/A 🌇N/A");
    }
}
