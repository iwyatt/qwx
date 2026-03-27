# Product Requirements Document (PRD) for `qwx`

## 1. Introduction

`qwx` (quick wx) is a command-line interface (CLI) application designed to provide users with quick, concise weather information directly in their terminal. It will display current weather conditions, today's forecast, and an extended 6-day forecast using a minimal, UTF-8 based format with intuitive emojis. The primary goal is to offer essential weather data efficiently without requiring a graphical user interface.

## 2. Goals

*   Provide immediate access to current weather conditions for a specified location.
*   Offer a clear, at-a-glance forecast for the current day.
*   Present a consolidated outlook for the next 6 days.
*   Utilize a visually minimal, UTF-8 and emoji-rich output format for quick comprehension.
*   Be efficient in resource usage (API calls, execution time).

## 3. User Stories

*   As a user, I want to quickly check the current temperature and conditions in my city from the terminal.
*   As a user, I want to see if it will rain today without opening a browser or a heavy application.
*   As a user, I want to plan my week by quickly glancing at the weather forecast for the next 6 days.
*   As a user, I want the weather information to be easy to read and understand at a glance, even with a lot of data.

## 4. Functionality

### 4.1. Weather Data Retrieval

*   `qwx` shall retrieve weather data from the Open-Meteo API for standard location queries (Zip Codes).
*   `qwx` shall retrieve aviation-specific weather data (METAR/TAF) from the NOAA Aviation Weather Center (AWC) API when an ICAO or FAA LID is provided.
*   **Location Priority:**
    1.  Command-line argument (Zip, ICAO, or FAA LID).
    2.  `default_location` specified in the configuration file.
    3.  `last_location` automatically stored in the configuration file.
*   `qwx` shall automatically update the `last_location` in the configuration file after every successful weather retrieval.

...

### 4.5. Configuration Management

*   `qwx` shall support a configuration file in the platform-specific standard location (using XDG Base Directory Specification on Linux).
*   The configuration file shall use the TOML format.
*   **Custom Formatting:** Users shall be able to define a custom format string for the Current Weather line using variables (e.g., `"{temp} {humidity}"`).
    *   **Safety Exception:** Custom formatting shall be ignored for Aviation Weather (METAR/TAF) to ensure no critical data is truncated or omitted.
*   **Future Goal:** Support custom formatting for Hourly and Daily forecast sections.

### 4.2. Output Display

`qwx` shall display weather information in up to three distinct sections. By default, only the **Current Weather** (or METAR) is displayed. Forecast sections (Daily/Hourly or TAF) must be explicitly requested via command-line arguments.

#### 4.2.1. Current Weather (Row 1) [Default]

This section shall display the following data points for the current conditions:

*   Temperature (°F)
*   Dew Point Temperature (°F)
*   Daily High Temperature (°F)
*   Daily Low Temperature (°F)
*   "Feels Like" Temperature (°F)
*   Weather Conditions (represented by specific emojis)
*   Wind Direction (represented by an arrow emoji, e.g., ↑ for North, → for East)
*   Wind Speed (in knots)
*   Sunrise Time
*   Sunset Time (separated from sunrise by an appropriate emoji)

#### 4.2.2. Hourly Forecast (Row 2+) [Optional]

This section shall display an hourly forecast for a specified number of intervals (defaulting to 12). **The hourly forecast shall begin from the first available interval strictly after the current weather time.** **Each hour shall be presented on its own distinct row** and shall include the same full set of data points as the Current Weather (Temperature, Dew Point, Hi/Lo, Weather Condition, Wind, Humidity, etc.) to ensure a complete picture of the upcoming weather.

#### 4.2.3. Daily Forecast (Row 3+) [Optional]

This section shall display a summary forecast for a specified number of days (defaulting to 7). **Each day shall be presented on its own distinct row**. The format for each day shall include:
*   Day of the Week (abbreviated)
*   High Temperature (°F)
*   Low Temperature (°F)
*   Weather Conditions (represented by specific emojis)
*   Chance of Precipitation (%)

Each forecast row for a single day shall strive to adhere to the 80-character limit.

#### 4.2.4. Aviation Weather (METAR/TAF)

*   **METAR (Current):** When an aviation identifier is detected, the current weather line shall parse and display the METAR data in an emoji-rich format similar to standard weather.
*   **TAF (Forecast):** When the TAF flag (`-t`, `--taf`) or the forecast flag (`-f`, `--forecast`) is used with an aviation identifier, each time block in the TAF shall be displayed as a separate row.
*   **Safety & Completeness:** **NO DATA TRUNCATION.** All critical fields from a METAR or TAF (Wind, Visibility, Weather, Sky Condition, Temperature/Dewpoint, Altimeter, and Remarks) must be preserved and displayed.
*   **Line Wrapping:** If a parsed METAR or TAF line exceeds the 80-character target, it must be wrapped to subsequent lines rather than truncated, ensuring all safety-critical information is visible.
*   **Safety Disclaimer:** All aviation-related output must include a clear disclaimer stating that the data is not for flight planning and that users should consult aviationweather.gov for official briefings.

#### 4.2.5. Output Formatting & Emojis

*   The output shall be minimal, UTF-8 encoded, and heavily leverage emojis for visual clarity.
*   Emoji mappings for weather conditions:
    *   ☀️: Clear
    *   ☁️: Clouds
    *   🌧️: Rain, Drizzle, Freezing Drizzle, Freezing Rain
    *   ⛈️: Thunderstorm
    *   ❄️: Snow, Snow Showers
    *   🌫️: Mist, Fog, Haze, Smoke, Dust, Sand, Ash
    *   🌪️: Squall, Tornado
    *   ❓: Unknown
*   The application shall strive to keep the total character count per output row under 80 characters for optimal terminal display.

### 4.3. Error Handling

*   `qwx` shall implement graceful error handling for issues such as network failures, invalid API keys, or invalid zip codes.
*   Upon encountering an error, `qwx` shall display a user-friendly error message and exit gracefully (non-zero exit code).

### 4.4. Non-Functional Requirements

*   **Performance:** Fast execution and response time.
*   **Reliability:** Robust handling of API responses and errors.
*   **Usability:** Clear, concise, and easy-to-read output.
*   **Maintainability:** Well-structured and commented Rust code.
*   **No Offline Mode:** The application does not require any offline caching or functionality.

## 5. Out of Scope

*   Interactive UI elements (beyond standard CLI input/output).
*   Saving user preferences (e.g., default zip code).
*   Multiple location support (initial version).
*   Graphical representations or charts.
*   Notifications.
*   Unit conversion within the application (Imperial is fixed).

---

*This document was generated by Gemini CLI.*