use std::env;
use std::fs;
use std::process::Command;
use serde_json::json;

const STATE_FILE: &str = "/tmp/loop_switch_state"; // File to store the state

fn main() {
    // Check if the program was called with the "toggle" argument
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "toggle" {
        toggle_state();
    } else {
        display_state();
    }
}

fn toggle_state() {
    // Read the current state from the file
    let is_on = match fs::read_to_string(STATE_FILE) {
        Ok(state) => state.trim() == "on",
        Err(_) => false, // Default to OFF if the file doesn't exist
    };

    // Toggle the state
    let new_state = !is_on;

    // Save the new state to the file
    let state_str = if new_state { "on" } else { "off" };
    if let Err(e) = fs::write(STATE_FILE, state_str) {
        eprintln!("Failed to write state file: {}", e);
        return;
    }

    // Execute the corresponding command
    if new_state {
        // Turn ON: Execute the command
        match Command::new("pw-loopback").arg("--name=MyLoopback").spawn() {
            Ok(_) => {
                let output_on = json!({
                    "text": "<span color=\"#00FF00\"></span>" 
                });
                println!("{}", output_on);// ON icon in green
            },
            Err(e) => eprintln!("Failed to execute command: {}", e),
        }
    } else {
        // Turn OFF: Kill the process
        match Command::new("pkill")
            .arg("-f")
            .arg("pw-loopback --name=MyLoopback")
            .spawn()
        {
            Ok(_) => {
                let output_off = json!({
                    "text": "<span color=\"#8B6914\"></span>"
                });
                println!("{}", output_off); // OFF icon in red
            },
            Err(e) => eprintln!("Failed to stop command: {}", e),
        }
    }
}

fn display_state() {
    // Read the current state from the file
    let is_on = match fs::read_to_string(STATE_FILE) {
        Ok(state) => state.trim() == "on",
        Err(_) => false, // Default to OFF if the file doesn't exist
    };

    // Display the current state
    if is_on {
        let output_on = json!({
            "text": "<span color=\"#00FF00\"></span>" //
        });
        println!("{}", output_on);// ON icon in green
    } else {
        let output_off = json!({
            "text": "<span color=\"#8B6914\"></span>" //
        });
        println!("{}", output_off); // OFF icon in red
    }
}
