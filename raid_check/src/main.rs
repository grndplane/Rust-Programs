// By MTE 3-21-2025 to report raid errors

use std::fs::File;
use std::io::{self, BufRead};
use std::process::Command;
use serde_json::json;

fn main() -> io::Result<()> {
    let path = "/proc/mdstat";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut status = None;

    for line in reader.lines() {
        let line = line?;
        // Find all bracket sets and take the last one that is all U/_
        let mut last_bracket = None;
        let mut search = line.as_str();
        while let Some(start) = search.find('[') {
            if let Some(end) = search[start..].find(']') {
                last_bracket = Some(&search[start + 1..start + end]);
                search = &search[start + end + 1..];
            } else {
                break;
            }
        }
        if let Some(s) = last_bracket {
            if s.chars().all(|c| c == 'U' || c == '_') {
                status = Some(s.to_string());
            }
        }
    }

    let status = status.unwrap_or_else(|| "??".to_string());

    // Get disk usage percent using df
    let df_output = Command::new("df")
        .args(&["-h", "--output=pcent", "/mnt/md0"])
        .output()
        .expect("Failed to run df");
    let df_str = String::from_utf8_lossy(&df_output.stdout);
    let mut lines = df_str.lines();
    let _header = lines.next(); // Skip header
    let usage_percent = lines.next().unwrap_or("").trim();

    // Parse the percentage number from the string (e.g., "44%" -> 44)
    let percent_num: u8 = usage_percent
        .trim_end_matches('%')
        .trim()
        .parse()
        .unwrap_or(0);

    // Choose color based on usage
    let color = if percent_num < 70 {
        "#C8EBC8" // Light Green
    } else {
        "#FF0000" // Red
    };

    // Unicode RAID icon
    let raid_icon = "🗄️";

    // Output as JSON with Pango markup for color
    let output = json!({
        "text": format!("{} <span color=\"{}\">{} {}</span>", raid_icon, color, usage_percent, status)
    });
    println!("{}", output);

    Ok(())
}
