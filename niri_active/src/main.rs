use std::process::Command;
use serde_json::Value;

fn main() {
    // Execute the `niri msg --json windows` command
    let window_output = Command::new("niri")
        .args(&["msg", "--json", "windows"])
        .output();

    let mut window_title = String::new();
    let mut focused_output = String::new(); // To store the monitor of the focused window

    match window_output {
        Ok(output) => {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                // Parse the JSON output
                match serde_json::from_str::<Value>(&stdout) {
                    Ok(json) => {
                        // Find the focused window
                        if let Some(focused_window) = json.as_array().and_then(|windows| {
                            windows.iter().find(|window| {
                                window.get("is_focused").and_then(Value::as_bool) == Some(true)
                            })
                        }) {
                            // Extract the title of the focused window
                            if let Some(title) = focused_window.get("title").and_then(Value::as_str) {
                                let simplified_title = title
                                    .rsplit(['-', ','].as_ref())
                                    .next()
                                    .unwrap_or(title)
                                    .trim();
                                window_title = simplified_title.to_string();
                            }

                            // Extract the monitor (output) of the focused window
                            if let Some(output) = focused_window.get("output").and_then(Value::as_str) {
                                focused_output = output.to_string();
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse JSON (windows): {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command (windows): {}", e);
        }
    }

    // Execute the `niri msg -j workspaces` command
    let workspace_output = Command::new("niri")
        .args(&["msg", "-j", "workspaces"])
        .output();

    let mut workspace_info = String::new();

    match workspace_output {
        Ok(output) => {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                // Parse the JSON output
                match serde_json::from_str::<Value>(&stdout) {
                    Ok(json) => {
                        // Find the workspace that is both active and focused
                        if let Some(active_workspace) = json.as_array().and_then(|workspaces| {
                            workspaces.iter().find(|workspace| {
                                workspace.get("is_active").and_then(Value::as_bool) == Some(true)
                                    && workspace.get("is_focused").and_then(Value::as_bool) == Some(true)
                            })
                        }) {
                            // Extract the name and output of the active workspace
                            if let Some(name) = active_workspace.get("name").and_then(Value::as_str) {
                                if let Some(output) = active_workspace.get("output").and_then(Value::as_str) {
                                    workspace_info = format!("{}/{}", name, output);
                                }
                            }
                        } else {
                            eprintln!("No matching active and focused workspace found.");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse JSON (workspaces): {}", e);
                    }
                }
            } else {
                eprintln!("Failed to parse workspace output as UTF-8.");
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command (workspaces): {}", e);
        }
    }

    // Combine the outputs
    if !window_title.is_empty() && !workspace_info.is_empty() {
        println!("{} {}", window_title, workspace_info);
    } else if !window_title.is_empty() {
        println!("{}", window_title);
    } else if !workspace_info.is_empty() {
        println!("{}", workspace_info);
    } else {
        eprintln!("No output available.");
    }
}