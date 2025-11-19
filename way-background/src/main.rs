use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;

fn main() {
    loop {
        // Kill all existing swaybg processes before starting new ones
        let _ = Command::new("pkill")
            .arg("swaybg")
            .output()
            .expect("Failed to kill swaybg processes");

        // Find two random images
        let image1 = find_random_image("/mnt/md0/Motorcycles/");
        let image2 = find_random_image("/mnt/md0/Motorcycles/");

        // Set a black background for both monitors
        set_black_background("DP-3");
        set_black_background("DP-4");
        sleep(Duration::from_secs(1)); // Wait for 1 second to ensure the black background is applied

        // Set the wallpaper for each monitor
        set_wallpaper("DP-3", &image1);
        set_wallpaper("DP-4", &image2);

        // Wait for 2 hours before changing the wallpapers again
        sleep(Duration::from_secs(7200));
    }
}

fn find_random_image(directory: &str) -> String {
    // Run the `find` command to list all files in the directory
    let find_output = Command::new("find")
        .arg(directory)
        .arg("-type")
        .arg("f")
        .output()
        .expect("Failed to execute find command");

    // Check if the `find` command succeeded
    if !find_output.status.success() {
        panic!("Find command failed: {}", String::from_utf8_lossy(&find_output.stderr));
    }

    // Debug: Print the output of the `find` command
    println!("Find output: {}", String::from_utf8_lossy(&find_output.stdout));

    // Check if the `find` command produced any output
    if find_output.stdout.is_empty() {
        panic!("No files found in the directory: {}", directory);
    }

    // Pipe the output of `find` into the `shuf` command
    let mut shuf_child = Command::new("shuf")
        .arg("-n1")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn shuf command");

    // Write the output of `find` to the stdin of `shuf`
    shuf_child.stdin.as_mut().unwrap().write_all(&find_output.stdout).expect("Failed to write to shuf stdin");

    // Wait for the `shuf` command to complete and capture its output
    let shuf_output = shuf_child.wait_with_output().expect("Failed to read shuf output");

    // Check if the `shuf` command succeeded
    if !shuf_output.status.success() {
        panic!("Shuf command failed: {}", String::from_utf8_lossy(&shuf_output.stderr));
    }

    // Debug: Print the output of the `shuf` command
    println!("Shuf output: {}", String::from_utf8_lossy(&shuf_output.stdout));

    // Check if the `shuf` command produced any output
    if shuf_output.stdout.is_empty() {
        panic!("Failed to select a random file from the directory: {}", directory);
    }

    // Return the randomly selected file path as a string
    String::from_utf8_lossy(&shuf_output.stdout).trim().to_string()
}

fn set_black_background(output: &str) {
    let _ = Command::new("swaybg")
        .arg("-o")
        .arg(output)
        .arg("-c")
        .arg("#000000")
        .arg("-m")
        .arg("fill")
        .spawn()
        .expect("Failed to set black background");
}

fn set_wallpaper(output: &str, image: &str) {
    let result = Command::new("swaybg")
        .arg("-o")
        .arg(output)
        .arg("-i")
        .arg(image)
        .arg("-m")
        .arg("center")
        .spawn();

    if let Err(e) = result {
        panic!("Failed to set wallpaper on {} with image {}: {}", output, image, e);
    }
}