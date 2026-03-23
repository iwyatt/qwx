use crate::model::WeatherReport;
use chrono::FixedOffset;
use std::collections::HashMap;
use regex::Regex;

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

pub fn format_weather_report(
    report: &WeatherReport,
    show_taf: bool,
    hourly_count: Option<u8>,
    daily_count: Option<u8>,
    custom_format: Option<&String>
) -> String {
    let mut output = Vec::new();

    if let Some(metar) = &report.metar {
        // Aviation Mode: Format METAR and TAF
        let header = format!("📍{} (METAR)", metar.station_id);
        output.extend(wrap_line(&format!("{} {}", header, metar.raw), 80));

        if show_taf {
            if let Some(taf) = &report.taf {
                output.push("Forecast (TAF):".to_string());
                for line in &taf.lines {
                    output.extend(wrap_line(line, 80));
                }
            } else {
                output.push("No TAF available for this station.".to_string());
            }
        }
    } else {
        // Standard Mode - Current Weather Line
        let default_current_weather_line = build_default_current_weather_line(report);
        let current_weather_line = format_current_weather_line(report, custom_format, &default_current_weather_line);
        output.push(current_weather_line);

        // Format hourly forecast if requested
        if let Some(count) = hourly_count {
            if !report.hourly_forecast.is_empty() {
                output.push("Hourly Forecast:".to_string());
                output.extend(format_hourly_forecast(report, count as usize));
            }
        }

        // Format daily forecast if requested
        if let Some(count) = daily_count {
            if !report.daily_forecast.is_empty() {
                output.push("Daily Forecast:".to_string());
                let daily_forecast_lines = report.daily_forecast.iter()
                    .take(count as usize)
                    .map(|entry| {
                        let day_name = entry.date.format("%a").to_string(); // Abbreviated day name
                        let condition_emoji = entry.weather_condition.emoji();
                        
                        let temp_display = format!("🌡️Hi:{:.0}F Lo:{:.0}F", entry.temp_max, entry.temp_min);
                        let dew_point_display = entry.dew_point.map(|dp| format!(" 💧{:.0}F", dp)).unwrap_or_else(|| "".to_string());
                        let humidity_val = entry.humidity.map(|h| format!(" {}%", h)).unwrap_or_else(|| " N/A%".to_string());
                        let precip_display = entry.precipitation_chance.map(|p| format!(" ☔{}%", p)).unwrap_or_else(|| "".to_string());
                        
                        let wind_emoji = entry.wind_deg.map(get_wind_direction_emoji).unwrap_or("❓");
                        let wind_speed_knots = entry.wind_speed.map(|s| s / 1.852).unwrap_or(0.0);
                        let wind_display = format!(" {}{:.0}kts", wind_emoji, wind_speed_knots);

                        let pressure_display = entry.pressure.map(|p| format!(" {:.1}Hg", (p as f64) * 0.02953)).unwrap_or_else(|| " N/AHg".to_string());

                        let local_timezone = report.timezone_offset.map(|offset| FixedOffset::east_opt(offset).unwrap_or(FixedOffset::east_opt(0).unwrap()))
                            .unwrap_or(FixedOffset::east_opt(0).unwrap());
                        let sunrise_display = entry.sunrise.map(|dt| format!(" 🌅{}", dt.with_timezone(&local_timezone).format("%H:%M"))).unwrap_or_else(|| " 🌅N/A".to_string());
                        let sunset_display = entry.sunset.map(|dt| format!(" 🌇{}", dt.with_timezone(&local_timezone).format("%H:%M"))).unwrap_or_else(|| " 🌇N/A".to_string());

                        format!("{}: {}{} {}{}{} {}{}{}{}", 
                            day_name, 
                            temp_display, 
                            dew_point_display,
                            condition_emoji, 
                            humidity_val, 
                            precip_display, 
                            wind_display, 
                            pressure_display, 
                            sunrise_display, 
                            sunset_display
                        )
                    })
                    .collect::<Vec<String>>();
                
                output.extend(daily_forecast_lines);
            }
        }
    }

    output.join("\n")
}

// Helper to format hourly forecast data
fn format_hourly_forecast(report: &WeatherReport, count: usize) -> Vec<String> {
    let mut hourly_output_lines = Vec::new();
    let local_timezone = report.timezone_offset.map(|offset| FixedOffset::east_opt(offset).unwrap_or(FixedOffset::east_opt(0).unwrap()))
        .unwrap_or(FixedOffset::east_opt(0).unwrap());

    // Filter hourly forecast to only include entries strictly in the future relative to report.datetime
    let filtered_forecast = report.hourly_forecast.iter()
        .filter(|entry| entry.time > report.datetime)
        .take(count);

    for entry in filtered_forecast {
        let time_display = entry.time.with_timezone(&local_timezone).format("%H:%M").to_string();
        
        let pressure_display = entry.pressure.map(|p| format!("{:.1}", (p as f64) * 0.02953));
        let wind_emoji = entry.wind_deg.map(get_wind_direction_emoji).unwrap_or("❓");
        let wind_speed_knots = entry.wind_speed / 1.852;
        let precip_display = entry.precipitation_chance.filter(|&p| p > 0).map(|p| format!(" ☔{}%", p)).unwrap_or_else(|| "".to_string());
        let dew_point_display = entry.dew_point.map(|dp| format!(" 💧{:.0}F", dp)).unwrap_or_else(|| "".to_string());

        let line = format!(
            "{} 🌡️{:.0}F{} {} {}%{} {}{:.0}kts {}Hg",
            time_display,
            entry.temperature,
            dew_point_display,
            entry.weather_condition.emoji(),
            entry.humidity.map(|h| h.to_string()).unwrap_or_else(|| "N/A".to_string()),
            precip_display,
            wind_emoji,
            wind_speed_knots,
            pressure_display.unwrap_or_else(|| "N/A".to_string())
        );
        hourly_output_lines.push(line);
    }
    hourly_output_lines
}

fn build_default_current_weather_line(report: &WeatherReport) -> String {
    let city_name = report.city_name.as_deref().unwrap_or("N/A");
    let state = report.state.as_deref().unwrap_or("");
    let country = report.country.as_deref().unwrap_or("N/A");

    let location_display = if state.is_empty() {
        format!("📍{}, {}", city_name, country)
    } else {
        format!("📍{}, {}, {}", city_name, state, country)
    };
    
    let temperature_f = report.temperature.round();
    let dew_point_f = report.dew_point.map(|t| t.round());

    let dew_point_display = match dew_point_f {
        Some(dp) => format!(" 💧{:.0}F", dp),
        None => "".to_string(),
    };

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

    let wind_speed_knots = report.wind_speed / 1.852;

    let local_timezone = report.timezone_offset.map(|offset| FixedOffset::east_opt(offset).unwrap_or(FixedOffset::east_opt(0).unwrap()))
        .unwrap_or(FixedOffset::east_opt(0).unwrap());

    let sunrise_time = report.sunrise.map(|dt| dt.with_timezone(&local_timezone).format("%H:%M").to_string()).unwrap_or_else(|| "N/A".to_string());
    let sunset_time = report.sunset.map(|dt| dt.with_timezone(&local_timezone).format("%H:%M").to_string()).unwrap_or_else(|| "N/A".to_string());

    let pressure_inhg = report.pressure.map(|p| (p as f64) * 0.02953);

    let pressure_display = match pressure_inhg {
        Some(p) => format!("{:.1}", p),
        None => "N/A".to_string(),
    };

    let current_precip_display = report.current_precipitation_chance
        .map(|p| format!(" ☔{}%", p))
        .unwrap_or_else(|| "".to_string());

    format!(
        "{} 🌡️{temp:.0}F{} {hilo_display} {condition_emoji} {humidity}%{current_precip_display} {wind_emoji}{wind_speed:.0}kts {pressure_display}Hg 🌅{sunrise_time} 🌇{sunset_time}",
        location_display,
        dew_point_display,
        temp = temperature_f,
        condition_emoji = report.weather_condition.emoji(),
        wind_emoji = report.wind_deg.map(|deg| get_wind_direction_emoji(deg)).unwrap_or("❓"),
        wind_speed = wind_speed_knots,
        humidity = report.humidity.map(|h| format!("{}", h)).unwrap_or_else(|| "N/A".to_string()),
        current_precip_display = current_precip_display,
        sunrise_time = sunrise_time,
        sunset_time = sunset_time
    )
}

fn format_current_weather_line(report: &WeatherReport, custom_format: Option<&String>, default_line: &str) -> String {
    // If METAR is present, always use the default line (safety exception)
    if report.metar.is_some() {
        return default_line.to_string();
    }

    if let Some(template) = custom_format {
        if template.is_empty() {
            return default_line.to_string();
        }

        let mut replacements = HashMap::new();

        // Location
        let city_name = report.city_name.as_deref().unwrap_or("N/A");
        let state = report.state.as_deref().unwrap_or("");
        let country = report.country.as_deref().unwrap_or("N/A");
        let location_display = if state.is_empty() {
            format!("{}, {}", city_name, country)
        } else {
            format!("{}, {}, {}", city_name, state, country)
        };
        replacements.insert("location", location_display);

        // Temperature (Fahrenheit)
        let temperature_f = report.temperature.round();
        replacements.insert("temp", format!("{:.0}", temperature_f));

        // Dew Point (Fahrenheit)
        let dew_point_f = report.dew_point.map(|t| t.round());
        replacements.insert("dew_point", dew_point_f.map_or_else(|| "N/A".to_string(), |dp| format!("{:.0}", dp)));

        // Hi/Lo Temps
        let (temp_max_f, temp_min_f) = if let Some(today) = report.daily_forecast.first() {
            (Some(today.temp_max.round()), Some(today.temp_min.round()))
        } else {
            (report.temp_max.map(|t| t.round()), report.temp_min.map(|t| t.round()))
        };
        let hilo_display = match (temp_max_f, temp_min_f) {
            (Some(max), Some(min)) => format!("Hi:{:.0}F Lo:{:.0}F", max, min),
            _ => "N/A".to_string(), // Simplified for template, use individual max/min for more detail
        };
        replacements.insert("hilo", hilo_display);
        replacements.insert("temp_max", temp_max_f.map_or_else(|| "N/A".to_string(), |t| format!("{:.0}", t)));
        replacements.insert("temp_min", temp_min_f.map_or_else(|| "N/A".to_string(), |t| format!("{:.0}", t)));

        // Feels Like Temp (Fahrenheit)
        let feels_like_f = report.feels_like.round();
        replacements.insert("feels_like", format!("{:.0}", feels_like_f));

        // Condition Emoji
        replacements.insert("condition_emoji", report.weather_condition.emoji().to_string());

        // Wind
        let wind_speed_knots = report.wind_speed / 1.852;
        replacements.insert("wind_speed", format!("{:.0}", wind_speed_knots));
        replacements.insert("wind_dir", report.wind_deg.map_or_else(|| "N/A".to_string(), |deg| format!("{}", deg)));
        replacements.insert("wind_emoji", report.wind_deg.map_or_else(|| "❓".to_string(), |deg| get_wind_direction_emoji(deg).to_string()));

        // Humidity
        replacements.insert("humidity", report.humidity.map_or_else(|| "N/A".to_string(), |h| format!("{}", h)));

        // Pressure (inHg)
        let pressure_inhg = report.pressure.map(|p| (p as f64) * 0.02953);
        replacements.insert("pressure", pressure_inhg.map_or_else(|| "N/A".to_string(), |p| format!("{:.1}", p)));

        // Sunrise/Sunset
        let local_timezone = report.timezone_offset.map(|offset| FixedOffset::east_opt(offset).unwrap_or(FixedOffset::east_opt(0).unwrap()))
            .unwrap_or(FixedOffset::east_opt(0).unwrap());
        replacements.insert("sunrise", report.sunrise.map_or_else(|| "N/A".to_string(), |dt| dt.with_timezone(&local_timezone).format("%H:%M").to_string()));
        replacements.insert("sunset", report.sunset.map_or_else(|| "N/A".to_string(), |dt| dt.with_timezone(&local_timezone).format("%H:%M").to_string()));
        
        let re = Regex::new(r"\{(\w+)\}").unwrap();
        let mut formatted_line = template.to_string();

        for cap in re.captures_iter(template) {
            let placeholder = &cap[0]; // e.g., "{temp}"
            let var_name = &cap[1];    // e.g., "temp"

            if let Some(replacement) = replacements.get(var_name) {
                formatted_line = formatted_line.replace(placeholder, replacement);
            }
        }
        formatted_line
    } else {
        default_line.to_string()
    }
}

fn wrap_line(text: &str, limit: usize) -> Vec<String> {
    if text.len() <= limit {
        return vec![text.to_string()];
    }

    let mut result = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut current_line = String::new();

    for word in words {
        let proposed_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };

        if proposed_line.len() > limit {
            if !current_line.is_empty() {
                result.push(current_line);
            }
            // Subsequent lines are indented
            current_line = format!("    {}", word);
        } else {
            current_line = proposed_line;
        }
    }

    if !current_line.is_empty() {
        result.push(current_line);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{WeatherCondition, WeatherReport};
    use chrono::{TimeZone, Utc, DateTime};

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
            temperature: 73.0, // Fahrenheit
            dew_point: Some(59.0),
            feels_like: 68.0,  // Fahrenheit
            temp_min: Some(64.0), // Fahrenheit
            temp_max: Some(82.0), // Fahrenheit
            pressure: Some(1013), // hPa
            humidity: Some(85),
            current_precipitation_chance: Some(30),
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
                    temp_max: 77.0,
                    temp_min: 59.0,
                    apparent_temperature_max: Some(75.0),
                    apparent_temperature_min: Some(57.0),
                    precipitation_chance: Some(30),
                    wind_speed: Some(18.52),
                    wind_deg: Some(270),
                    sunrise: Some(Utc.with_ymd_and_hms(2023, 1, 2, 6, 31, 0).unwrap()),
                    sunset: Some(Utc.with_ymd_and_hms(2023, 1, 2, 17, 46, 0).unwrap()),
                    humidity: Some(80),
                    pressure: Some(1013),
                    dew_point: Some(58.0),
                },
                DailyForecastEntry {
                    date: Utc.with_ymd_and_hms(2023, 1, 3, 0, 0, 0).unwrap().date_naive(),
                    weather_condition: WeatherCondition::Rain,
                    temp_max: 68.0,
                    temp_min: 50.0,
                    apparent_temperature_max: Some(66.0),
                    apparent_temperature_min: Some(48.0),
                    precipitation_chance: Some(80),
                    wind_speed: Some(25.0),
                    wind_deg: Some(180),
                    sunrise: Some(Utc.with_ymd_and_hms(2023, 1, 3, 6, 32, 0).unwrap()),
                    sunset: Some(Utc.with_ymd_and_hms(2023, 1, 3, 17, 47, 0).unwrap()),
                    humidity: Some(90),
                    pressure: Some(1010),
                    dew_point: Some(60.0),
                },
                 DailyForecastEntry {
                    date: Utc.with_ymd_and_hms(2023, 1, 4, 0, 0, 0).unwrap().date_naive(),
                    weather_condition: WeatherCondition::Clear,
                    temp_max: 72.0,
                    temp_min: 54.0,
                    apparent_temperature_max: Some(70.0),
                    apparent_temperature_min: Some(52.0),
                    precipitation_chance: Some(10),
                    wind_speed: Some(10.0),
                    wind_deg: Some(0),
                    sunrise: Some(Utc.with_ymd_and_hms(2023, 1, 4, 6, 33, 0).unwrap()),
                    sunset: Some(Utc.with_ymd_and_hms(2023, 1, 4, 17, 48, 0).unwrap()),
                    humidity: Some(60),
                    pressure: Some(1015),
                    dew_point: Some(50.0),
                },
                DailyForecastEntry {
                    date: Utc.with_ymd_and_hms(2023, 1, 5, 0, 0, 0).unwrap().date_naive(),
                    weather_condition: WeatherCondition::Snow,
                    temp_max: 41.0,
                    temp_min: 28.0,
                    apparent_temperature_max: Some(35.0),
                    apparent_temperature_min: Some(22.0),
                    precipitation_chance: Some(90),
                    wind_speed: Some(30.0),
                    wind_deg: Some(45),
                    sunrise: Some(Utc.with_ymd_and_hms(2023, 1, 5, 6, 34, 0).unwrap()),
                    sunset: Some(Utc.with_ymd_and_hms(2023, 1, 5, 17, 49, 0).unwrap()),
                    humidity: Some(85),
                    pressure: Some(1005),
                    dew_point: Some(25.0),
                },
                DailyForecastEntry {
                    date: Utc.with_ymd_and_hms(2023, 1, 6, 0, 0, 0).unwrap().date_naive(),
                    weather_condition: WeatherCondition::Thunderstorm,
                    temp_max: 82.0,
                    temp_min: 64.0,
                    apparent_temperature_max: Some(85.0),
                    apparent_temperature_min: Some(67.0),
                    precipitation_chance: Some(70),
                    wind_speed: Some(40.0),
                    wind_deg: Some(225),
                    sunrise: Some(Utc.with_ymd_and_hms(2023, 1, 6, 6, 35, 0).unwrap()),
                    sunset: Some(Utc.with_ymd_and_hms(2023, 1, 6, 17, 50, 0).unwrap()),
                    humidity: Some(75),
                    pressure: Some(1008),
                    dew_point: Some(70.0),
                },
                DailyForecastEntry {
                    date: Utc.with_ymd_and_hms(2023, 1, 7, 0, 0, 0).unwrap().date_naive(),
                    weather_condition: WeatherCondition::Mist,
                    temp_max: 59.0,
                    temp_min: 46.0,
                    apparent_temperature_max: Some(57.0),
                    apparent_temperature_min: Some(44.0),
                    precipitation_chance: Some(20),
                    wind_speed: Some(15.0),
                    wind_deg: Some(315),
                    sunrise: Some(Utc.with_ymd_and_hms(2023, 1, 7, 6, 36, 0).unwrap()),
                    sunset: Some(Utc.with_ymd_and_hms(2023, 1, 7, 17, 51, 0).unwrap()),
                    humidity: Some(95),
                    pressure: Some(1012),
                    dew_point: Some(55.0),
                },
            ],
            hourly_forecast: Vec::new(),
            metar: None,
            taf: None,
        };

        let formatted = format_weather_report(&report, false, None, Some(6), None);
        let lines: Vec<&str> = formatted.lines().collect();

        // Check expected content of the first line (current weather)
        assert!(lines[0].contains("📍Testville, TS, US 🌡️73F 💧59F  Hi:77F Lo:59F ☀️ 85% ☔30% ⬅️10kts 29.9Hg 🌅06:30 🌇17:45"));

        // Check expected content of daily forecast lines
        assert!(lines[1].contains("Daily Forecast:"));
        assert!(lines[2].contains("Mon: 🌡️Hi:77F Lo:59F 💧58F ☁️ 80% ☔30%  ⬅️10kts 29.9Hg 🌅06:31 🌇17:46"));
    }

    #[test]
    fn test_format_weather_report_optional_fields() {
        let report = WeatherReport {
            city_name: None,
            state: None,
            country: None,
            temperature: 68.0, // Fahrenheit
            dew_point: None,
            feels_like: 64.0,  // Fahrenheit
            temp_min: None,
            temp_max: None,
            pressure: None,
            humidity: None,
            current_precipitation_chance: None,
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
            hourly_forecast: Vec::new(), // Added this line
            metar: None,
            taf: None,
        };

        let formatted = format_weather_report(&report, false, None, None, None); // Updated
        assert_eq!(formatted, "📍N/A, N/A 🌡️68F  Hi:N/A Lo:N/A ❓ N/A% ❓0kts N/AHg 🌅N/A 🌇N/A");
    }

    #[test]
    fn test_format_weather_report_hourly_forecast() {
        use crate::model::HourlyForecastEntry;

        let report = WeatherReport {
            city_name: Some("Hourlyville".to_string()),
            state: None,
            country: Some("US".to_string()),
            temperature: 50.0, // Fahrenheit
            dew_point: Some(41.0),
            feels_like: 46.0,  // Fahrenheit
            temp_min: Some(41.0),
            temp_max: Some(59.0),
            pressure: Some(1010),
            humidity: Some(70),
            current_precipitation_chance: Some(10),
            weather_condition: WeatherCondition::Clear,
            wind_speed: 9.26, // km/h (5 knots)
            wind_deg: Some(180),
            sunrise: Some(Utc.with_ymd_and_hms(2023, 1, 1, 7, 0, 0).unwrap()),
            sunset: Some(Utc.with_ymd_and_hms(2023, 1, 1, 18, 0, 0).unwrap()),
            datetime: Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap(),
            timezone_offset: Some(0), // UTC
            latitude: 0.0,
            longitude: 0.0,
            daily_forecast: Vec::new(),
            hourly_forecast: vec![
                HourlyForecastEntry {
                    time: DateTime::from_timestamp(1672582400, 0).unwrap(), // Jan 1, 2023 14:13:20 UTC
                    temperature: 41.0, // Fahrenheit
                    weather_condition: WeatherCondition::Clear,
                    precipitation_chance: Some(0),
                    wind_speed: 5.0,
                    wind_deg: Some(0),
                    dew_point: Some(35.0),
                    feels_like: Some(38.0),
                    humidity: Some(80),
                    pressure: Some(1012),
                },
            ],
            metar: None,
            taf: None,
        };

        let formatted = format_weather_report(&report, false, Some(1), None, None);
        let lines: Vec<&str> = formatted.lines().collect();

        assert!(lines.len() >= 2);
        assert_eq!(lines[1], "Hourly Forecast:");
        assert!(lines[2].contains("14:13 🌡️41F 💧35F ☀️ 80% ⬆️3kts 29.9Hg"));
    }

    #[test]
    fn test_format_weather_report_hourly_forecast_filtering() {
        use crate::model::HourlyForecastEntry;

        let report_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();

        let report = WeatherReport {
            city_name: Some("Filterville".to_string()),
            state: None,
            country: Some("US".to_string()),
            temperature: 50.0,
            dew_point: Some(41.0),
            feels_like: 46.0,
            temp_min: None,
            temp_max: None,
            pressure: Some(1010),
            humidity: Some(70),
            current_precipitation_chance: None,
            weather_condition: WeatherCondition::Clear,
            wind_speed: 10.0,
            wind_deg: Some(180),
            sunrise: None,
            sunset: None,
            datetime: report_time,
            timezone_offset: Some(0),
            latitude: 0.0,
            longitude: 0.0,
            daily_forecast: Vec::new(),
            hourly_forecast: vec![
                HourlyForecastEntry {
                    time: Utc.with_ymd_and_hms(2023, 1, 1, 11, 0, 0).unwrap(), // Past
                    temperature: 40.0,
                    weather_condition: WeatherCondition::Clear,
                    precipitation_chance: None,
                    wind_speed: 5.0,
                    wind_deg: Some(0),
                    dew_point: None,
                    feels_like: None,
                    humidity: None,
                    pressure: None,
                },
                HourlyForecastEntry {
                    time: Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap(), // Present (should be filtered if using > )
                    temperature: 42.0,
                    weather_condition: WeatherCondition::Clear,
                    precipitation_chance: None,
                    wind_speed: 5.0,
                    wind_deg: Some(0),
                    dew_point: None,
                    feels_like: None,
                    humidity: None,
                    pressure: None,
                },
                HourlyForecastEntry {
                    time: Utc.with_ymd_and_hms(2023, 1, 1, 13, 0, 0).unwrap(), // Future
                    temperature: 44.0,
                    weather_condition: WeatherCondition::Clear,
                    precipitation_chance: None,
                    wind_speed: 5.0,
                    wind_deg: Some(0),
                    dew_point: None,
                    feels_like: None,
                    humidity: None,
                    pressure: None,
                },
            ],
            metar: None,
            taf: None,
        };

        let formatted = format_weather_report(&report, false, Some(12), None, None);
        let lines: Vec<&str> = formatted.lines().collect();

        // Should only show the entry for 13:00
        assert!(lines.iter().any(|line| line.contains("13:00 🌡️44F")));
        assert!(!lines.iter().any(|line| line.contains("11:00 🌡️40F")));
        assert!(!lines.iter().any(|line| line.contains("12:00 🌡️42F")));
    }
}
