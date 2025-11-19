use std::fs;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use serde_json::json;

fn main() {
    // Dynamically find the correct hwmon directory for Intel Arc GPU
    let mut hwmon_path = String::new();
    for i in 1..=8 {
        let path = format!("/sys/class/hwmon/hwmon{}", i);
        if fs::metadata(&path).is_ok() && fs::metadata(format!("{}/energy1_input", path)).is_ok() {
            hwmon_path = path;
            break;
        }
    }

    // Check if HWMON is set, exit with error if not found
    if hwmon_path.is_empty() {
        eprintln!("Error: Could not find the correct hwmon directory.");
        std::process::exit(1);
    }

    // Read initial power value
    let power1 = read_energy(&hwmon_path);

    // Wait for a short interval
    sleep(Duration::from_millis(250));

    // Read second power value
    let power2 = read_energy(&hwmon_path);

    // Calculate power consumption
    // Subtracting power1 from power2 gives energy used in the interval
    // Dividing by 252525 converts to watts (adjust this value if needed for your specific GPU)
    let power = (power2 - power1) / 252525;

    // Get GPU information from glxinfo
    let gpu = get_gpu_name();
    
    // Choose color based on usage
    let color = if power < 185 {
        "#C8EBC8" // Light Green
    } else {
        "#FF0000" // Red
    };

    // Unicode Monitor icon
    let icon = "🖥️";

    // Output as JSON with Pango markup for color
    let output = json!({
        "text": format!("{} <span color=\"{}\">{} {}W</span>", icon, color, gpu, power)
    });
    println!("{}", output);


}


fn read_energy(hwmon_path: &str) -> i64 {
    let energy_path = format!("{}/energy1_input", hwmon_path);
    match fs::read_to_string(&energy_path) {
        Ok(content) => content.trim().parse::<i64>().unwrap_or(0),
        Err(_) => {
            eprintln!("Error: Could not read energy value from {}", energy_path);
            std::process::exit(1);
        }
    }
}

fn get_gpu_name() -> String {
    let output = Command::new("glxinfo")
        .arg("-B")
        .output()
        .expect("Failed to execute glxinfo command");

    if let Ok(stdout) = String::from_utf8(output.stdout) {
        for line in stdout.lines() {
            if line.contains("OpenGL renderer") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 6 {
                    return parts[6].to_string();
                }
            }
        }
    }

    "Unknown GPU".to_string()
}
