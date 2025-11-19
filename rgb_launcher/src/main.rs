use std::{process::Command, thread, time::Duration};

fn is_running(process: &str) -> bool {
    let output = Command::new("pgrep")
        .arg("-x")
        .arg(process)
        .output()
        .expect("Failed to run pgrep");
    !output.stdout.is_empty()
}

fn killall(process: &str) {
    let output = Command::new("pgrep")
        .arg("-x")
        .arg(process)
        .output()
        .expect("Failed to run pgrep");
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if let Ok(pid) = line.parse::<i32>() {
            // Use libc to send SIGTERM
            unsafe {
                libc::kill(pid, libc::SIGTERM);
            }
        }
    }
}

fn main() {
    // Wait 20 seconds before starting
    thread::sleep(Duration::from_secs(20));

    // Turn off Evolution tasks
    killall("evolution-alarm");

    // Policy Kit
    if !is_running("polkit-gnome-authentication-agent-1") {
        Command::new("/usr/lib/policykit-1-gnome/polkit-gnome-authentication-agent-1")
            .spawn()
            .expect("Failed to start polkit-gnome-authentication-agent-1");
    }

    // OpenRGB
    killall("openrgb");
    if !is_running("openrgb") {
        Command::new("openrgb")
            .args(&["--startminimized", "--profile", "mike1"])
            .spawn()
            .expect("Failed to start openrgb");
    }

    // ckb-next
    killall("ckb-next");
    if !is_running("ckb-next") {
        Command::new("ckb-next")
            .arg("--background")
            .spawn()
            .expect("Failed to start ckb-next");
    }
}