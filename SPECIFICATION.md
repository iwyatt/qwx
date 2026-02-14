# Specification Document for `qwx`

## 1. Overview

This document specifies the technical design and architecture for `qwx`, a Rust-based command-line interface (CLI) application for displaying weather information. It details the module structure, data flows, external integrations, and output formatting.

## 2. Architecture

`qwx` will follow a modular architecture, separating concerns into distinct components:

*   **`main` module:** Handles CLI argument parsing, orchestrates the flow, and calls other modules.
*   **`weather_api` module:** Manages communication with the OpenWeatherMap API, including request formulation, error handling, and data parsing.
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
|   weather_api   |----->| OpenWeatherMap  |
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

*   **OpenWeatherMap API:** Used for fetching current weather and forecast data.
    *   Likely using the "One Call API 3.0" for comprehensive data.
    *   API Key will be read from the environment variable `OPENWEATHERMAP_API_KEY`.
*   **Rust Crates:**
    *   `clap`: For robust command-line argument parsing (e.g., zip code input).
    *   `reqwest`: For making asynchronous HTTP requests to the OpenWeatherMap API.
    *   `tokio`: Asynchronous runtime for `reqwest`.
    *   `serde` / `serde_json`: For serializing/deserializing JSON data from the API.
    *   `chrono`: For handling date and time, especially for sunrise/sunset and hourly forecasts.

## 4. Command-Line Interface (CLI)

The application will be invoked as `qwx`.

### 4.1. Arguments

*   `<zip_code>` (Required): The 5-digit US zip code for which to fetch weather.

**Example Usage:**
`qwx 90210`

## 5. Data Structures (within `model` module)

Data structures will be defined to represent the parsed API responses. Key structs will include:

*   `CurrentWeather`: Holds current temperature, feels_like, weather condition details, wind speed/direction, sunrise/sunset.
*   `HourlyForecast`: Holds temperature, conditions, wind, etc. for a specific hour.
*   `DailyForecast`: Holds high/low temperatures, conditions, precipitation chance for a specific day.
*   `WeatherCondition`: Enum or struct to map OpenWeatherMap condition codes to internal representations and associated emojis.

## 6. `weather_api` Module Details

### 6.1. API Endpoint Construction

*   The module will construct the appropriate OpenWeatherMap API URL using the provided zip code and API key.
*   It will handle unit conversion parameters to request Imperial units directly from the API if supported, or handle conversion internally if not.

### 6.2. Error Handling

*   Network errors, invalid API responses (e.g., non-200 status codes), or malformed JSON will be caught.
*   Errors will be converted into a `qwx` specific `Error` type (e.g., `enum QwxError { NetworkError, ApiError, ParseError }`).
*   In case of an API error (e.g., invalid zip code, invalid API key), specific error messages will be returned to `main` for graceful display.

### 6.3. Data Parsing

*   The module will parse the JSON response from OpenWeatherMap into the internal data structures defined in the `model` module.

## 7. `display` Module Details

### 7.1. Output Logic

The `display` module will be responsible for orchestrating the three-row output as follows:

*   **Row 1: Current Weather**
    *   Format: `Temp°F (FeelsLike°F) | Cond_Emoji Wind_Dir_Emoji Wind_Speed_knots | 🌅 HH:MM 🌇 HH:MM`
    *   Example: `72°F (68°F) | ☀️ ↓ 10kts | 🌅 06:30 🌇 19:45`
*   **Row 2: Today's Hourly Forecast**
    *   To be implemented with 3 or 6-hour increments. This will require checking how OpenWeatherMap provides hourly data and whether it aligns with the "same data points as Current Weather" requirement within the 80-character limit. If full details exceed the limit, a condensed format will be used (e.g., `HH:MM Temp°F Cond_Emoji`).
    *   Example (condensed): `10:00 70°F ☀️ | 13:00 75°F ⛅ | 16:00 72°F 🌧️`
*   **Row 3: Next 6 Days Forecast**
    *   Format per day: `Day_of_Week Hi°F/Lo°F Cond_Emoji Precip_Chance%`
    *   Example: `Mon 75°F/60°F ☀️ 10% | Tue 70°F/55°F ☁️ 20% | ...`

### 7.2. Emoji Mapping

*   A clear mapping function or lookup table will translate OpenWeatherMap weather condition codes to the specified UTF-8 emojis.
*   Wind direction (e.g., N, NE, E, SE, S, SW, W, NW) will be mapped to appropriate arrow emojis (e.g., ↑, ↗, →, ↘, ↓, ↙, ←, ↖).

### 7.3. Character Limit Enforcement

*   The display logic will dynamically adjust content or truncate information to adhere to the target 80-character row limit where feasible without losing critical information. This might involve shortening day names or omitting less critical data points if necessary for very dense forecasts.

## 8. Error Handling (Application Level)

*   The `main` function will catch any `QwxError` propagated from `weather_api`.
*   A user-friendly message corresponding to the error type will be printed to `stderr`.
*   The application will exit with a non-zero status code (e.g., `exit(1)`).

## 9. Future Considerations (Out of Scope for initial version)

*   Default location configuration (e.g., via a config file like `~/.config/qwx/config.toml`).
*   Geolocation based on IP address.
*   Detailed hourly forecast for more than today.
*   Support for multiple unit systems via CLI flag.
*   Interactive modes.

---

*This document was generated by Gemini CLI.*