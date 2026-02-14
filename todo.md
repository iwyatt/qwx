Ok, we've got a working proof of concept. Im not sure the output makes complete sense and Im not confiden that the zip we provided correctly resolved to Maple Valley, WA.
- Address concerns about output clarity and zip code resolution.
- Display sunrise and sunset times in the local timezone of the provided zip code.
- Add atmospheric pressure (converted to inches of mercury) to the output.
- Implement fetching of state information for US zip codes:
  - Modify `get_lat_lon_from_zip` to first call the `/zip` endpoint to get `lat`, `lon`, `city_name`, and `country`.
  - Then, make a second API call to the "reverse geocoding" endpoint (`http://api.openweathermap.org/geo/1.0/reverse?lat={lat}&lon={lon}&limit=1&appid={API_KEY}`) to get more detailed information, including the state.
  - Update `CurrentWeather` struct, its constructor, and `display.rs` to correctly handle and display the fetched state.