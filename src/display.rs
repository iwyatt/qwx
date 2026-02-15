use crate::model::WeatherReport;
use chrono::{FixedOffset, Utc};

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

pub fn format_weather_report(report: &WeatherReport) -> String {
    let city_name = report.city_name.as_deref().unwrap_or("N/A");
    let country = report.country.as_deref().unwrap_or("N/A");
    
    // Convert temperature to Fahrenheit if needed (Open-Meteo returns Celsius)
    let temperature_f = (report.temperature * 9.0 / 5.0) + 32.0;
    let _feels_like_f = (report.feels_like * 9.0 / 5.0) + 32.0;
    let temp_min_f = report.temp_min.map(|t| (t * 9.0 / 5.0) + 32.0);
    let temp_max_f = report.temp_max.map(|t| (t * 9.0 / 5.0) + 32.0);

    // Convert wind speed from km/h to knots
    let wind_speed_knots = report.wind_speed / 1.852;

    let local_timezone = report.timezone_offset.map(|offset| FixedOffset::east_opt(offset).unwrap_or(FixedOffset::east_opt(0).unwrap()))
        .unwrap_or(FixedOffset::east_opt(0).unwrap());


    let sunrise_time = report.sunrise.map(|dt| dt.with_timezone(&local_timezone).format("%H:%M").to_string()).unwrap_or_else(|| "N/A".to_string());
    let sunset_time = report.sunset.map(|dt| dt.with_timezone(&local_timezone).format("%H:%M").to_string()).unwrap_or_else(|| "N/A".to_string());

    let pressure_inhg = report.pressure.map(|p| (p as f64) * 0.02953);

    format!(
        "📍{city_name}, {country} {temp:.0}F Hi:{temp_max:.0}F Lo:{temp_min:.0}F {condition_emoji} {wind_emoji}{wind_speed:.0}kts 💧{humidity}% {pressure_inhg:.2}Hg  🌅{sunrise_time} 🌇{sunset_time}",
        city_name = city_name,
        country = country,
        temp = temperature_f,
        temp_max = temp_max_f.map(|t| format!("{:.0}", t)).unwrap_or_else(|| "N/A".to_string()),
        temp_min = temp_min_f.map(|t| format!("{:.0}", t)).unwrap_or_else(|| "N/A".to_string()),
        condition_emoji = report.weather_condition.emoji(),
        wind_emoji = report.wind_deg.map(|deg| get_wind_direction_emoji(deg)).unwrap_or("❓"),
        wind_speed = wind_speed_knots,
        humidity = report.humidity.map(|h| format!("{}", h)).unwrap_or_else(|| "N/A".to_string()),
        pressure_inhg = pressure_inhg.map(|p| format!("{:.2}", p)).unwrap_or_else(|| "N/A".to_string()),
        sunrise_time = sunrise_time,
        sunset_time = sunset_time
    )
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
        let report = WeatherReport {
            city_name: Some("Testville".to_string()),
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
        };

        let formatted = format_weather_report(&report);
        // Expected values:
        // temp: 22.5 C = 72.5 F
        // temp_max: 28.0 C = 82.4 F
        // temp_min: 18.0 C = 64.4 F
        // wind_speed: 18.52 km/h = 10 knots
        // pressure: 1013 hPa = 29.91 inHg
        assert_eq!(formatted, "📍Testville, US 73F Hi:82F Lo:64F ☀️ ⬅️10kts 💧85% 29.91Hg  🌅06:30 🌇17:45");
    }

    #[test]
    fn test_format_weather_report_optional_fields() {
        let report = WeatherReport {
            city_name: None,
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
        };

        let formatted = format_weather_report(&report);
        assert_eq!(formatted, "📍N/A, N/A 68F Hi:N/AF Lo:N/AF ❓ ❓0kts 💧N/A% N/AHg  🌅N/A 🌇N/A");
    }
}
