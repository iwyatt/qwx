mod model;
mod weather_api;
mod display;

use clap::{Parser, ValueEnum};
use weather_api::WeatherApiError;

/// A quick weather CLI app written in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The location (Zip Code, ICAO, or FAA LID) for which to fetch weather information.
    location: String,

    /// The weather API provider to use.
    #[arg(short, long, value_enum, default_value_t = ApiProvider::OpenMeteo)]
    api_provider: ApiProvider,

    /// Display the 6-day forecast or TAF.
    #[arg(short = 't', long = "taf")]
    taf: bool,

    /// Display the today's hourly forecast.
    #[arg(short = 'H', long)]
    hourly: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum ApiProvider {
    /// Use OpenWeatherMap API
    OpenWeatherMap,
    /// Use Open-Meteo API
    OpenMeteo,
}

#[tokio::main]
async fn main() -> Result<(), WeatherApiError> {
    let cli = Cli::parse();

    let provider = match cli.api_provider {
        ApiProvider::OpenWeatherMap => weather_api::WeatherApiProvider::OpenWeatherMap,
        ApiProvider::OpenMeteo => weather_api::WeatherApiProvider::OpenMeteo,
    };

    let weather_report = weather_api::get_weather(&cli.location, provider).await?;
    
    println!("{}", display::format_weather_report(&weather_report, cli.taf, cli.hourly));

    Ok(())
}
