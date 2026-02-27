mod config;
mod model;
mod weather_api;
mod display;

use clap::{Parser, ValueEnum};
use crate::config::Config;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use regex::Regex;

/// A quick weather CLI app written in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The location (Zip Code, ICAO, or FAA LID) for which to fetch weather information.
    /// If not provided, it will try to use `default_location` or `last_location` from the config.
    location: Option<String>,

    /// The weather API provider to use.
    #[arg(short, long, value_enum, default_value_t = ApiProvider::OpenMeteo)]
    api_provider: ApiProvider,

    /// Display the forecast (hourly or daily) or TAF.
    /// For zip codes: -f h [count] or -f d [count].
    /// For aviation: -f or -t.
    #[arg(short = 'f', long = "forecast", num_args = 0..=2)]
    forecast: Option<Vec<String>>,

    /// Shortcut for TAF (aviation only).
    #[arg(short = 't', long = "taf")]
    taf: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum ApiProvider {
    /// Use OpenWeatherMap API
    OpenWeatherMap,
    /// Use Open-Meteo API
    OpenMeteo,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let mut config = Config::load()?;

    let location_str: String;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner:.green} {msg}")?);
    spinner.set_message("Detecting location...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    if let Some(cli_location) = cli.location {
        location_str = cli_location;
        spinner.finish_and_clear();
    } else if let Some(default_loc) = config.default_location.clone() {
        location_str = default_loc;
        spinner.finish_and_clear();
    } else if let Some(last_loc) = config.last_location.clone() {
        location_str = last_loc;
        spinner.finish_and_clear();
    } else {
        // Attempt to auto-detect location from IP
        match weather_api::get_location_from_ip().await {
            Ok(ip_location) => {
                location_str = ip_location;
                spinner.finish_with_message(format!("Auto-detected location from IP: {}", location_str));
            },
            Err(e) => {
                spinner.finish_and_clear();
                anyhow::bail!("No location provided via CLI or config, and IP-based auto-detection failed: {}", e);
            }
        }
    }

    let provider = match cli.api_provider {
        ApiProvider::OpenWeatherMap => weather_api::WeatherApiProvider::OpenWeatherMap,
        ApiProvider::OpenMeteo => weather_api::WeatherApiProvider::OpenMeteo,
    };

    let weather_spinner = ProgressBar::new_spinner();
    weather_spinner.set_style(ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner:.green} {msg}")?);
    weather_spinner.set_message("Fetching weather data...");
    weather_spinner.enable_steady_tick(Duration::from_millis(100));

    let weather_report = weather_api::get_weather(&location_str, provider).await?;
    
    weather_spinner.finish_and_clear();
    
    // Save the last successful location
    config.last_location = Some(location_str.clone());
    config.save()?;

    let mut show_taf = cli.taf;
    let mut hourly_count = None;
    let mut daily_count = None;

    if let Some(args) = cli.forecast {
        if args.is_empty() {
            let zip_regex = Regex::new(r"^\d{5}$").unwrap();
            let aviation_regex = Regex::new(r"^[a-zA-Z0-9]{3,4}$").unwrap();
            if aviation_regex.is_match(&location_str) && !zip_regex.is_match(&location_str) {
                show_taf = true;
            } else {
                anyhow::bail!("Please specify 'h' (hourly) or 'd' (daily) for the forecast, e.g., -f h 12");
            }
        } else {
            match args[0].as_str() {
                "h" => {
                    let count = args.get(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(12);
                    hourly_count = Some(count);
                }
                "d" => {
                    let count = args.get(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(7);
                    daily_count = Some(count);
                }
                _ => {
                    let zip_regex = Regex::new(r"^\d{5}$").unwrap();
                    let aviation_regex = Regex::new(r"^[a-zA-Z0-9]{3,4}$").unwrap();
                    if aviation_regex.is_match(&location_str) && !zip_regex.is_match(&location_str) {
                        show_taf = true;
                    } else {
                        anyhow::bail!("Invalid forecast option: '{}'. Use 'h' or 'd'.", args[0]);
                    }
                }
            }
        }
    }

    println!("{}", display::format_weather_report(
        &weather_report,
        show_taf,
        hourly_count,
        daily_count,
        config.current_format.as_ref()
    ));

    Ok(())
}
