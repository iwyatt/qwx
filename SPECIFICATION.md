# Specification Document for `qwx`

## 1. Overview

This document specifies the technical design and architecture for `qwx`, a Rust-based command-line interface (CLI) application for displaying weather information. It details the module structure, data flows, external integrations, and output formatting.

## 2. Architecture

`qwx` will follow a modular architecture, separating concerns into distinct components:

*   **`main` module:** Handles CLI argument parsing, orchestrates the flow, and calls other modules.
*   **`weather_api` module:** Manages communication with the Open-Meteo API, including request formulation, error handling, and data parsing.
*   **`display` module:** Formats and prints the weather data to the console, adhering to the specified output style and emoji usage.
*   **`model` module:** Defines data structures for weather information received from the API and used internally.

```
+----------------+
|     main.rs    |
| (CLI Entrypoint)|
+--------+-------+
         |
         v
+--------+--------+      +-----------------+
|   weather_api   |----->|   Open-Meteo    |
| (API Interaction)|      |       API       |
+--------+--------+      +-----------------+
         |
         v
+--------+--------+
|     model       |
| (Data Structures)|
+--------+--------+
         |
         v
+--------+--------+
|     display     |
| (Output Formatting)|
+--------+--------+
         |
         v
+----------------+
|    Console     |
| (User Output)  |
+----------------+
```

## 3. External Dependencies

*   **Open-Meteo API:** Used for fetching current weather and forecast data for Zip Codes.
*   **NOAA Aviation Weather Center (AWC) API:** Used for fetching METAR and TAF data for ICAO/FAA identifiers.
*   **Rust Crates:**
    *   `clap`: For robust command-line argument parsing.
    *   `reqwest`: For making asynchronous HTTP requests.
    *   `tokio`: Asynchronous runtime.
    *   `serde` / `serde_json`: For serializing/deserializing data.
    *   `chrono`: For handling date and time.
    *   `directories`: For cross-platform configuration directory discovery.
    *   `toml`: For parsing and serializing the configuration file.

...

## 10. Configuration Management

### 10.1. Storage Location
*   **Linux:** `~/.config/qwx/config.toml`
*   **macOS:** `~/Library/Application Support/qwx/config.toml`
*   **Windows:** `%AppData%\qwx\config.toml`

### 10.2. Structure (TOML)
```toml
default_location = "90210"
last_location = "KSEA"
current_format = "📍{location} {temp}F {condition_emoji} {wind_emoji}{wind_speed}kts"
```

### 10.3. Templating Engine
*   The `display` module will implement a simple parser to replace `{variable}` tokens with data from the `WeatherReport`.
*   Supported variables: `{location}`, `{temp}`, `{hilo}`, `{temp_max}`, `{temp_min}`, `{feels_like}`, `{condition_emoji}`, `{wind_speed}`, `{wind_dir}`, `{wind_emoji}`, `{humidity}`, `{pressure}`, `{sunrise}`, `{sunset}`.
*   If `metar` is present in the `WeatherReport`, the template is bypassed in favor of the safety-critical raw output logic.

## 4. Command-Line Interface (CLI)

The application will be invoked as `qwx`.

### 4.1. Arguments

*   `<location>` (Required):
    *   **Zip Code:** 5 digits (e.g., `90210`).
    *   **Aviation ID:** 3-4 character alpha-numeric string (e.g., `KSEA`, `SEA`, `S60`).
*   `-f`, `--forecast` [Sub-options]: Optional. Enables forecast output.
    *   **For Zip Codes:** Requires sub-options:
        *   `h [count]`: Hourly forecast for `count` intervals (default 12).
        *   `d [count]`: Daily forecast for `count` intervals (default 7).
    *   **For Aviation IDs:** Enables TAF output (ignores sub-options).
*   `-t`, `--taf`: Shortcut for `--forecast` when using an aviation ID.

**Example Usage:**
*   `qwx 90210` (Standard current weather)
*   `qwx 90210 -f d 5` (Standard current weather + 5-day forecast)
*   `qwx 90210 -f h 6` (Standard current weather + 6-hour forecast)
*   `qwx KSEA` (METAR current weather)
*   `qwx KSEA -f` (METAR + TAF)
*   `qwx KSEA -t` (METAR + TAF)

## 5. Data Structures (within `model` module)

Data structures will be defined to represent the parsed API responses. Key structs will include:

*   `CurrentWeather`: Holds current temperature, dew_point, feels_like, weather condition details, wind speed/direction, sunrise/sunset.
*   `HourlyForecast`: Holds temperature, conditions, wind, etc. for a specific hour.
*   `DailyForecast`: Holds high/low temperatures, conditions, precipitation chance for a specific day.
*   `MetarReport`: Holds station ID, observation time, wind, visibility, sky condition, temp/dewpoint, altimeter, and raw remarks.
*   `TafReport`: Holds a collection of `TafLine` entries, each representing a forecast time block.
*   `WeatherCondition`: Updated to include aviation-specific conditions derived from METAR/TAF codes (e.g., `FG`, `BR`, `TSRA`).

## 6. `weather_api` Module Details

### 6.1. API Interaction

*   **Open-Meteo Client:** Used if the location matches a 5-digit zip code.
*   **NOAA AWC Client:** Used if the location matches a 3-4 character alpha-numeric aviation identifier.
    *   METAR Endpoint: `https://aviationweather.gov/api/data/metar?ids=[ID]&format=raw`
    *   TAF Endpoint: `https://aviationweather.gov/api/data/taf?ids=[ID]&format=raw`
*   The module will utilize the `open-meteo-rs` crate to interact with the Open-Meteo API.
*   Geocoding will be handled by the `open-meteo-rs` geocoding functionality, using the provided zip code or location name to obtain latitude and longitude.

### 6.2. Error Handling

*   Network errors, invalid API responses, or malformed JSON will be caught.
*   Errors will be converted into a `qwx` specific `Error` type (e.g., `enum QwxError { NetworkError, ApiError, ParseError, LocationResolutionError }`).
*   In case of an API error or failure in any stage of location resolution, specific user-friendly error messages will be returned for graceful display.

### 6.3. Data Parsing

*   The `open-meteo-rs` crate will handle the parsing of API responses into its own data structures, which will then be mapped to the internal `WeatherReport` model.

## 7. `display` Module Details

### 7.1. Output Logic

The `display` module will be responsible for orchestrating the output rows based on user-provided flags:

*   **Row 1: Current Weather** (Always displayed)
    *   Format: `📍[Location] [Temp]°F ([DewPoint]°F) Hi:[High]°F Lo:[Low]°F [Cond_Emoji] [Humidity]% [Precip_Chance]% [Wind_Emoji][Speed]kts [Pressure]Hg  🌅 HH:MM 🌇 HH:MM`
    *   Example: `📍Jackson, US 39°F (34°F) Hi:39°F Lo:38°F ☀️ ↙️3kts 💧63% 30.03Hg  🌅07:35 🌇18:08`
*   **Row 2+: Hourly Forecast** (Displayed if `-f h` is set)
    *   **Filtering:** The `display` module filters the hourly forecast to only include entries whose time is strictly later than the report's current `datetime`.
    *   Each hourly interval is displayed on its own row.
    *   Format: Same as Current Weather (Row 1), prefixed with the time.
    *   Example: `10:00 🌡️70F 💧60F Hi:75F Lo:68F ☀️ ↙️5kts 💧65% 30.01Hg`
*   **Row 3+: Daily Forecast** (Displayed if `-f d` is set)
    *   Each day of the forecast will be presented on its own distinct row.
    *   Format per day: `Day_of_Week Hi°F Lo°F Cond_Emoji Precip_Chance%`
    *   Example: `Mon 75°F 60°F ☀️ 10%`
    *   Each daily forecast row shall strive to adhere to the 80-character limit.

#### 7.1.4. Aviation Output (METAR/TAF)

*   **Wrapping Algorithm:**
    1.  If a formatted line exceeds 80 characters, split it into chunks.
    2.  Ensure chunks are split at space characters to avoid breaking words or codes.
    3.  Indent subsequent chunks by 4 spaces to visually group them with the parent row.
    4.  **Crucial:** Do not omit any tokens from the raw input (e.g., RMK section of METAR).

### 7.2. Emoji Mapping

*   **Weather Condition Mappings:**
    *   ☀️: Clear
    *   ☁️: Clouds
    *   🌧️: Rain, Drizzle, Freezing Drizzle, Freezing Rain
    *   ⛈️: Thunderstorm
    *   ❄️: Snow, Snow Showers
    *   🌫️: Mist, Fog, Haze, Smoke, Dust, Sand, Ash
    *   🌪️: Squall, Tornado
    *   ❓: Unknown
*   **Aviation-specific mappings:** (e.g., `FEW` -> 🌤️, `SCT` -> ⛅, `BKN` -> ☁️, `OVC` -> ☁️).
*   **Visibility:** 💧 (if low), 🌫️ (if mist/fog).
*   **Wind direction:** (e.g., N, NE, E, SE, S, SW, W, NW) will be mapped to appropriate arrow emojis (e.g., ↑, ↗, →, ↘, ↓, ↙, ←, ↖).

### 7.3. Character Limit Enforcement

*   The display logic will dynamically adjust content or truncate information to adhere to the target 80-character row limit where feasible without losing critical information. This might involve shortening day names or omitting less critical data points if necessary for very dense forecasts.

## 8. Error Handling (Application Level)

*   The `main` function will catch any `QwxError` propagated from `weather_api`.
*   A user-friendly message corresponding to the error type will be printed to `stderr`.
*   The application will exit with a non-zero status code (e.g., `exit(1)`).

## 9. Future Considerations (Out of Scope for initial version)

*   Default location configuration (e.g., via a config file like `~/.config/qwx/config.toml`).

*   Detailed hourly forecast for more than today.
*   Support for multiple unit systems via CLI flag.
*   Interactive modes.

---

*This document was generated by Gemini CLI.*