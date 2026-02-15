use crate::model::CurrentWeather;
use chrono::FixedOffset;

// Helper function to get wind direction emoji
pub fn get_wind_direction_emoji(degrees: i32) -> &'static str {
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

pub fn format_current_weather(current: &CurrentWeather) -> String {
    let local_timezone = FixedOffset::east_opt(current.timezone_offset_seconds).unwrap_or(FixedOffset::east_opt(0).unwrap());

    let sunrise_time = current.sunrise.with_timezone(&local_timezone).format("%H:%M").to_string();
    let sunset_time = current.sunset.with_timezone(&local_timezone).format("%H:%M").to_string();

    let pressure_inhg = (current.pressure_hpa as f64) * 0.02953;

    format!(
        "📍{city_name}, {country} {temp:.0}F Hi:{temp_max:.0}F Lo:{temp_min:.0}F {condition_emoji} {wind_emoji}{wind_speed:.0}kts 💧{humidity}% {pressure_inhg:.2}Hg  🌅{sunrise_time} 🌇{sunset_time}",
        city_name = current.city_name,
        country = current.country,
        temp = current.temperature,
        temp_max = current.temp_max,
        temp_min = current.temp_min,
        condition_emoji = current.condition.emoji(),
        wind_emoji = get_wind_direction_emoji(current.wind_deg),
        wind_speed = current.wind_speed,
        humidity = current.humidity,
        pressure_inhg = pressure_inhg,
        sunrise_time = sunrise_time,
        sunset_time = sunset_time
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::WeatherCondition;
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
        assert_eq!(get_wind_direction_emoji(-10), "❓"); // Out of range
    }

    #[test]
    fn test_format_current_weather() {
        let current = CurrentWeather {
            city_name: "Testville".to_string(),
            country: "US".to_string(),
            temperature: 72.5,
            feels_like: 68.1,
            temp_min: 65.0,
            temp_max: 78.0,
            humidity: 85,
            pressure_hpa: 1013,
            condition: WeatherCondition::Clear,
            wind_speed: 10.3,
            wind_deg: 270, // West
            sunrise: Utc.with_ymd_and_hms(2023, 1, 1, 6, 30, 0).unwrap(),
            sunset: Utc.with_ymd_and_hms(2023, 1, 1, 17, 45, 0).unwrap(),
            timezone_offset_seconds: 0,
        };

        let formatted = format_current_weather(&current);
        assert!(formatted.contains("📍Testville, US T:72F Hi:78F Lo:65F ☀️ ⬅️10kts 💧85% P:29.91inHg  🌅06:30 🌇17:45"));
        assert!(formatted.len() <= 120);
    }
}
