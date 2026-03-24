# qwx (Quick Weather)

`qwx` is a high-performance, minimal, and emoji-rich weather CLI application written in Rust. It's designed to provide you with the most critical weather information at a glance, directly in your terminal, with a strong emphasis on brevity and UTF-8 visuals.

Example:
```
> qwx seattle -f h 3

📍Seattle, Washington, US 🌡️42F 💧33F  Hi:54F Lo:41F ☁️ 70% ☔1% ↗️1kts 29.8Hg 🌅07:03 🌇19:27
Hourly Forecast:
23:00 🌡️41F 💧33F ☁️ 73% ☔2% ↘️0kts 29.8Hg
00:00 🌡️41F 💧34F ☁️ 75% ☔4% ↘️1kts 29.7Hg
01:00 🌡️41F 💧34F ☁️ 76% ☔8% ↘️1kts 29.7Hg
```

## Features

-   **Brevity by Design:** Get current conditions and forecasts in a single, well-formatted line of text.
-   **Aviation Weather Support:** Built-in support for fetching and parsing METAR and TAF data using ICAO or FAA identifiers (e.g., `KSEA`, `S60`).
-   **Multi-location Search:** Support for Zip Codes, Aviation IDs, and City/State searches (e.g., "Seattle, WA").
-   **Daily and Hourly Forecasts:** Optional forecast views for standard locations and TAFs for aviation.
-   **Customizable Formatting:** Support for custom format strings in your configuration file.
-   **Cross-platform Configuration:** Uses standard platform-specific configuration directories (e.g., XDG on Linux).

## Differentiation

While many CLI weather tools focus on large, ASCII-art heavy displays, `qwx` is built for speed and integration into a developer's workflow. Its unique ability to seamlessly switch between standard consumer weather (Open-Meteo) and professional aviation weather (NOAA AWC) makes it an essential tool for both daily commuters and pilots.

## Installation

```bash
# Clone the repository
git clone https://github.com/google-gemini/qwx.git
cd qwx

# Build and install
cargo install --path .
```

## Usage

### Standard Weather
```bash
# Current weather for a zip code
qwx 90210

# Current weather and 5-day forecast
qwx 90210 -f d 5

# Current weather for a city
qwx "Seattle, WA"
```

### Aviation Weather
```bash
# METAR for an ICAO airport code
qwx KSEA

# METAR and TAF (forecast)
qwx KSEA -f
# or use the shortcut
qwx KSEA -t
```

### Configuration
`qwx` stores its configuration in `~/.config/qwx/config.toml` (on Linux).

```toml
default_location = "90210"
```

## Disclosure

This project was co-written with  Google Gemini, showcasing the capabilities of AI-driven software development assistance in building comprehensive, performant CLI tools.

## License

This project is licensed under the [MIT License](license.md).
