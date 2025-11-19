// Rust weather script by M.T.E. and Copilot, using moon-phases CLI for moon icon

use chrono::{Local, NaiveTime, Datelike};
use reqwest::blocking::Client;
use serde_json::Value;
use std::fs;
use std::process::{self, Command};

fn get_sunrise_sunset_real(lat: f64, lon: f64) -> (NaiveTime, NaiveTime) {
    let url = format!(
        "https://api.sunrise-sunset.org/json?lat={}&lng={}&formatted=0",
        lat, lon
    );
    let client = Client::new();
    let resp = client.get(&url).send();

    match resp {
        Ok(response) => {
            if let Ok(json) = response.json::<Value>() {
                let sunrise_utc = json["results"]["sunrise"].as_str().unwrap_or("");
                let sunset_utc = json["results"]["sunset"].as_str().unwrap_or("");
                let sunrise = sunrise_utc.parse::<chrono::DateTime<chrono::Utc>>().ok()
                    .map(|dt| dt.with_timezone(&Local).time())
                    .unwrap_or(NaiveTime::from_hms_opt(6, 0, 0).unwrap());
                let sunset = sunset_utc.parse::<chrono::DateTime<chrono::Utc>>().ok()
                    .map(|dt| dt.with_timezone(&Local).time())
                    .unwrap_or(NaiveTime::from_hms_opt(18, 0, 0).unwrap());
                (sunrise, sunset)
            } else {
                (NaiveTime::from_hms_opt(6, 0, 0).unwrap(), NaiveTime::from_hms_opt(18, 0, 0).unwrap())
            }
        }
        Err(_) => (
            NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
        ),
    }
}

/// Get moon phase icon using the moon-phases CLI
fn moon_phase_icon_cli() -> String {
    let today = Local::now().date_naive();
    let date_str = format!("{}-{}-{}", today.year(), today.month(), today.day());
    let output = Command::new("moon-phases")
        .args(["--face-emoji", &date_str])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        }
        _ => "🌙".to_string(), // fallback icon
    }
}

fn main() {
    let lat = 34.0686;
    let lon = -117.9389;
    let (sunrise, sunset) = get_sunrise_sunset_real(lat, lon);
    let now = Local::now().time();
    let is_night = now < sunrise || now > sunset;

    // Load API key
    let api_key = fs::read_to_string(format!("{}/.owm-key", std::env::var("HOME").unwrap()))
        .expect("Failed to read API key")
        .trim()
        .to_string();

    // Configuration
    let city_name = "West Covina";
    let country_code = "US";
    let units = "imperial";
    let lang = "en";

    // Build API URL
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units={}&lang={}",
        city_name, country_code, api_key, units, lang
    );

    // Fetch weather data
    let client = Client::new();
    let response = client.get(&url).send();

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let json: Value = resp.json().unwrap();
                process_weather_data(json, is_night);
            } else {
                eprintln!("Failed to fetch weather data: {}", resp.status());
                process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn process_weather_data(json: Value, is_night: bool) {
    let weather = &json["weather"][0];
    let main = &json["main"];
    let wind = &json["wind"];

    let description = weather["description"].as_str().unwrap_or("Unknown");
    let temp = main["temp"].as_f64().unwrap_or(0.0);
    let humidity = main["humidity"].as_u64().unwrap_or(0);
    let wind_speed = wind["speed"].as_f64().unwrap_or(0.0);
    let wind_deg = wind["deg"].as_f64().unwrap_or(0.0);

    let icon = if is_night {
        moon_phase_icon_cli()
    } else {
        match weather["id"].as_i64().unwrap_or(0) {
            200..=232 => "🌩️".to_string(),
            300..=321 => "🌦️".to_string(),
            500..=504 => "🌦️".to_string(),
            511 => "🌧️".to_string(),
            520..=531 => "⛈️".to_string(),
            600..=622 => "🌨️".to_string(),
            701..=771 => "🌫".to_string(),
            781 => "🌪️".to_string(),
            800 => "🌞".to_string(),
            801 => "☁️".to_string(),
            802 => "🌥️".to_string(),
            803..=804 => "🌤️".to_string(),
            _ => "".to_string(),
        }
    };

    let wind_arrow = match wind_deg as i32 {
        0..=22 | 338..=360 => "󰮽",
        23..=67 => "󰮼",
        68..=112 => "󰮺",
        113..=157 => "󰮶",
        158..=202 => "󰮷",
        203..=247 => "󰮵",
        248..=292 => "󰮹",
        293..=337 => "󰮻",
        _ => "?",
    };

    let spc = ", ";
    let tempu = temp as u64;
    let wind_speedu = wind_speed as u64;

    if wind_speedu < 10 {
        println!("{} {}{}{} {}%💧", icon, description, spc, tempu, humidity);
    } else {
        println!(
            "{} {}{}{} {}%💧,{}mph {} ",
            icon, description, spc, tempu, humidity, wind_speedu, wind_arrow
        );
    }
}