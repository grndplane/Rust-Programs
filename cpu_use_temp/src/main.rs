use std::fs;
use std::process::Command;
use std::time::Duration;
use serde_json::json;

fn main() {
    // Get initial CPU stats
    let initial_cpu_stats = get_cpu_stats();

    // Wait for a short duration to calculate CPU usage
    std::thread::sleep(Duration::from_millis(500));

    // Get CPU stats again
    let final_cpu_stats = get_cpu_stats();

    // Calculate CPU usage
    let cpu_usage = calculate_cpu_usage(&initial_cpu_stats, &final_cpu_stats);

    // Execute the `sensors` command to get temperature
    let output = Command::new("sensors")
        .arg("-f") // Use Fahrenheit
        .output();

    match output {
        Ok(output) => {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                // Filter lines containing "Package id 0:"
                if let Some(line) = stdout.lines().find(|line| line.contains("Package id 0:")) {
                    // Extract the temperature value
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Some(temp) = parts.get(3) {
                        // Clean the temperature string
                        let temp = temp
                            .trim_start_matches('+') // Remove leading '+'
                            .trim_end_matches("°F") // Remove trailing '°F'
                            .split('.') // Handle decimal values
                            .next() // Take the integer part
                            .unwrap_or(""); // Default to an empty string if split fails

                        // Parse the cleaned temperature string
                        match temp.parse::<u32>() {
                            Ok(num) => {
                               
                            let color:&'static str  = if num < 122 {
                                "#C8EBC8" // Light Green
                            } else if num < 158 {
                                "#FFFF00" // Yellow
                            } else {
                                "#FF0000" // Red
                            };

                            // Unicode Monitor icon
                            let icon = "🧇";

                            // Output as JSON with Pango markup for color
                            let output = json!({
                                "text": format!("{} <span color=\"{}\">{}% {:>3.1}</span>", icon, color, cpu_usage, num as u32)
                            });
                            println!("{}", output);

                            }
                            Err(e) => {
                                eprintln!("Error parsing temperature: {}", e);
                            }
                        }
                    } else {
                        eprintln!("Failed to parse temperature from line: {}", line);
                    }
                } else {
                    eprintln!("No line containing 'Package id 0:' found.");
                }
            } else {
                eprintln!("Failed to parse command output as UTF-8.");
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
        }
    }
}

// Function to read CPU stats from /proc/stat
fn get_cpu_stats() -> Vec<u64> {
    if let Ok(contents) = fs::read_to_string("/proc/stat") {
        if let Some(line) = contents.lines().find(|line| line.starts_with("cpu ")) {
            return line
                .split_whitespace()
                .skip(1) // Skip the "cpu" label
                .filter_map(|value| value.parse::<u64>().ok())
                .collect();
        }
    }
    vec![]
}

// Function to calculate CPU usage percentage
fn calculate_cpu_usage(
    initial: &[u64],
    final_stats: &[u64],
) -> u32 {
    if initial.len() == final_stats.len() && !initial.is_empty() {
        let total_initial: u64 = initial.iter().sum();
        let total_final: u64 = final_stats.iter().sum();
        let idle_initial = initial[3]; // Idle time is the 4th value
        let idle_final = final_stats[3];

        let total_diff = total_final.saturating_sub(total_initial);
        let idle_diff = idle_final.saturating_sub(idle_initial);

        if total_diff > 0 {
            let usage = 100 * (total_diff - idle_diff) / total_diff;
            return usage as u32;
        }
    }
    0
}
