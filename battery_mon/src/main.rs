// Witten by: Michael T. Edwards and copilot.
// Date: 2025-04-26

use std::fs;
use std::io::{self, Write};

const STATE_FILE: &str = "/tmp/battery_state.txt";

fn main() {
    // Read battery information
    let battery_info = read_battery_info("/sys/class/power_supply/hidpp_battery_0/uevent");

    // If battery information is not available, try to load it from the saved state
    let battery_info = battery_info.or_else(load_battery_state);

    // Display mouse battery capacity and status
    if let Some((capacity, status)) = battery_info {
        // Save the current state for future use
        save_battery_state(capacity, &status).unwrap_or_else(|err| {
            eprintln!("Warning: Could not save battery state: {}", err);
        });

        // Display the battery information
        let batt = if capacity >= 80 {
            "🔋"
        } else {
            "🪫"
        };
        println!("{}", batt);
    } else {
        eprintln!("Error: Could not read battery information.");
        std::process::exit(1);
    }
}

fn read_battery_info(path: &str) -> Option<(u8, String)> {
    if let Ok(content) = fs::read_to_string(path) {
        let mut capacity = 0;
        let mut status = String::new();

        for line in content.lines() {
            if line.starts_with("POWER_SUPPLY_CAPACITY=") {
                if let Ok(value) = line.replace("POWER_SUPPLY_CAPACITY=", "").parse::<u8>() {
                    capacity = value;
                }
            } else if line.starts_with("POWER_SUPPLY_STATUS=") {
                status = line.replace("POWER_SUPPLY_STATUS=", "");
            }
        }

        if !status.is_empty() {
            return Some((capacity, status));
        }
    }

    None
}

fn save_battery_state(capacity: u8, status: &str) -> io::Result<()> {
    let mut file = fs::File::create(STATE_FILE)?;
    writeln!(file, "capacity={}", capacity)?;
    writeln!(file, "status={}", status)?;
    Ok(())
}

fn load_battery_state() -> Option<(u8, String)> {
    if let Ok(content) = fs::read_to_string(STATE_FILE) {
        let mut capacity = 0;
        let mut status = String::new();

        for line in content.lines() {
            if line.starts_with("capacity=") {
                if let Ok(value) = line.replace("capacity=", "").parse::<u8>() {
                    capacity = value;
                }
            } else if line.starts_with("status=") {
                status = line.replace("status=", "");
            }
        }

        if !status.is_empty() {
            return Some((capacity, status));
        }
    }

    None
}
