# qwx (Quick Weather)

`qwx` is a high-performance, minimal, and emoji-rich weather CLI application written in Rust. It's designed to provide you with the most critical weather information at a glance, directly in your terminal, with a strong emphasis on brevity and UTF-8 visuals.

Examples:
```
> qwx seattle -f h 3

📍Seattle, Washington, US 🌡️42F 💧33F  Hi:54F Lo:41F ☁️ 70% ☔1% ↗️1kts 29.8Hg 🌅07:03 🌇19:27
Hourly Forecast:
23:00 🌡️41F 💧33F ☁️ 73% ☔2% ↘️0kts 29.8Hg
00:00 🌡️41F 💧34F ☁️ 75% ☔4% ↘️1kts 29.7Hg
01:00 🌡️41F 💧34F ☁️ 76% ☔8% ↘️1kts 29.7Hg
```

```
> qwx KSEA

📍KSEA (METAR) SPECI KSEA 241719Z 31007KT 1SM R16L/5500VP6000FT -RA BR BKN003
    OVC011 07/06 A2977 RMK AO2 BKN003 V SCT P0005 T00670061 $
```

```
> qwx KSEA -f

📍KSEA (METAR) SPECI KSEA 241719Z 31007KT 1SM R16L/5500VP6000FT -RA BR BKN003
    OVC011 07/06 A2977 RMK AO2 BKN003 V SCT P0005 T00670061 $
Forecast (TAF):
TAF KSEA 241458Z 2415/2518 00000KT 2SM -RA BR OVC030
FM241600 15008KT 4SM -RA BR OVC025
FM241800 17010KT 6SM -RA BR OVC020
FM242200 19016G30KT 6SM -RA BR OVC020
FM250300 20015G25KT P6SM VCSH BKN035
FM250900 19012G18KT P6SM VCSH BKN040
FM251200 20010KT P6SM VCSH BKN040

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

## Attribution

Weather data by [Open-Meteo.com](https://open-meteo.com/).

## Disclosure

This project was co-written with  Google Gemini, showcasing the capabilities of AI-driven software development assistance in building comprehensive, performant CLI tools.

## License

This project is licensed under the [MIT License](license.md).
