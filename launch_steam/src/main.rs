use std::process::{Command, exit};

fn main() {
    // Stop picom
    if let Err(e) = Command::new("pkill")
        .arg("-x")
        .arg("picom")
        .status()
    {
        eprintln!("Failed to stop picom: {}", e);
    }

    // Launch Steam and wait for it to close
    if let Ok(status) = Command::new("steam").status() {
        if status.success() {
            println!("Steam has exited.");
        } else {
            eprintln!("Steam exited with a non-zero status.");
        }
    } else {
        eprintln!("Failed to launch Steam.");
    }

    // Restart picom
    if let Err(e) = Command::new("picom")
        .arg("--config")
        .arg(format!("{}/.config/leftwm/themes/current/picom.conf", std::env::var("HOME").unwrap()))
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        eprintln!("Failed to restart picom: {}", e);
    }

    exit(0);
}