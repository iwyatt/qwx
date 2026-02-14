mod model;
mod weather_api;
mod display;

use clap::Parser;
use weather_api::WeatherApiError;

/// A quick weather CLI app written in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The zip code for which to fetch weather information.
    zip_code: String,
}

#[tokio::main]
async fn main() -> Result<(), WeatherApiError> {
    let cli = Cli::parse();

    let current_weather = weather_api::fetch_and_parse_current_weather(&cli.zip_code).await?;
    
    println!("{}", display::format_current_weather(&current_weather));

    Ok(())
}
