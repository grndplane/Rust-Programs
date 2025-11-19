use chrono::DateTime;
use reqwest::blocking::get;
use serde_json::Value;
use std::env;
use std::error::Error;

fn get_compass_direction(azimuth: f64) -> &'static str {
    let directions = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let index = (((azimuth as i32 + 22) % 360) / 45) as usize;
    directions[index]
}

fn get_satellite_pass(
    sat_id: u32,
    sat_name: &str,
    lat: f64,
    lon: f64,
    alt: u32,
    api_key: &str,
) -> Result<(), Box<dyn Error>> {
    let base_url = "https://api.n2yo.com/rest/v1/satellite/visualpasses";
    let url = format!(
        "{}/{}/{}/{}/{}/7/3/?apiKey={}",
        base_url, sat_id, lat, lon, alt, api_key
    );

    // Fetch data from API
    let response = get(&url)?.text()?;
    let json: Value = serde_json::from_str(&response)?;

    // Check if API returned any passes
    if json["passes"].is_null() || json["passes"].as_array().unwrap().is_empty() {
        eprintln!(
            "No pass found for {} for the given location and time range.",
            sat_name
        );
        return Ok(());
    }

    // Process each pass
    let mut count = 0;
    for pass in json["passes"].as_array().unwrap() {
        if count >= 2 {
            break;
        }

        let start_time = pass["startUTC"].as_i64().unwrap_or(0);
        let duration = pass["duration"].as_i64().unwrap_or(0);
        let max_az = pass["maxAz"].as_f64().unwrap_or(0.0);
        let max_el = pass["maxEl"].as_f64().unwrap_or(0.0);

        // Convert UTC timestamp to readable date format
        let start_date = DateTime::from_timestamp(start_time, 0)
            .map(|dt| dt.format("%d %b %I:%M %p").to_string())
            .unwrap_or_else(|| "Invalid date".to_string());

        // Calculate duration in minutes and seconds
        let duration_min = duration / 60;
        let duration_sec = duration % 60;

        // Get compass direction from azimuth
        let direction = get_compass_direction(max_az);

        // Print formatted output
        println!(
            "{:<7} | {} | {:2}m{:02}s | {:2} {:2.0}°",
            sat_name, start_date, duration_min, duration_sec, direction, max_el
        );

        count += 1;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Set default parameters
    let latitude = 34.06862;
    let longitude = -117.93895;
    let altitude = 156;

    // Satellite IDs
    let iss_id = 25544;

    // API key
    let api_key =
        env::var("N2YO_API_KEY").unwrap_or_else(|_| "EDQVYF-BMEVMQ-3NXQTD-4QXV".to_string());

    // Print header
    println!("Satellite | Date & Time    | Duration | Direction & Elevation");
    println!("---------+----------------+----------+--------------------------");
    get_satellite_pass(iss_id, "ISS", latitude, longitude, altitude, &api_key)?;
    println!("---------+----------------+----------+--------------------------");

    Ok(())
}
