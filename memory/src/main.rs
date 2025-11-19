use std::env;
use std::process::{Command, Stdio};
use serde_json::json;

fn parse_used_gb(s: &str) -> f32 {
    // Accepts "2.3G", "800M", "123K", etc.
    let s = s.trim();
    if let Some(num) = s[..s.len()-1].parse::<f32>().ok() {
        match s.chars().last().unwrap_or('G') {
            'G' => num,
            'M' => num / 1024.0,
            'K' => num / (1024.0 * 1024.0),
            _ => num,
        }
    } else {
        0.0
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--popup" {
        let ps_output = Command::new("ps")
            .args(&["axch", "-o", "cmd:10,pmem", "k", "-pmem"])
            .output();

        match ps_output {
            Ok(output) => {
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    let formatted_output: String = stdout
                        .lines()
                        .map(|line| format!("{}%", line))
                        .collect::<Vec<String>>()
                        .join("\n");
                    let _ = Command::new("notify-send")
                        .arg("Memory (%)")
                        .arg(&formatted_output)
                        .output();
                }
            }
            Err(e) => eprintln!("Failed to execute ps command: {}", e),
        }
    } else {
        let free_output = Command::new("free")
            .args(&["-h", "--si"])
            .stdout(Stdio::piped())
            .output();

        match free_output {
            Ok(output) => {
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    // Extract the memory usage from the "Mem:" line
                    if let Some(line) = stdout.lines().find(|line| line.starts_with("Mem:")) {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() > 2 {
                            // println!("🤯{}", parts[2]); // Print the used memory
                            // Choose color based on usage

                            let num: f32 = parse_used_gb(parts[2]);

                            let used_gb = parse_used_gb(parts[2]);

                            let color:&'static str  = if num < 24.0 {
                                "#C8EBC8" // Light Green
                            } else {
                                "#FF0000" // Red
                            };

                            // Unicode Monitor icon
                            let icon = "🤯";

                            // Output as JSON with Pango markup for color
                            let output = json!({
                                "text": format!("{} <span color=\"{}\">{:.1}G</span>", icon, color, used_gb)
                            });
                            println!("{}", output);

                        } else {
                            eprintln!("Unexpected format in free output.");
                        }
                    }
                } else {
                    eprintln!("Failed to parse free output as UTF-8.");
                }
            }
            Err(e) => eprintln!("Failed to execute free command: {}", e),
        }
    }
}


