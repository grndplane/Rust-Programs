use std::fs::{self, File};
use std::io::{self, Write};
use std::process::{Command, exit};
use std::thread::sleep;
use std::time::Duration;
use chrono::Local;

fn main() -> io::Result<()> {
    let lock_file = "/tmp/dvrProcessing.lock";
    let in_file = std::env::args().nth(1).expect("Input file not provided");
    let tmp_file = format!("{}.mp4", in_file);
    let dvr_post_log = "/tmp/dvrProcessing.log";
    let handbrake = "/usr/bin/HandBrakeCLI";

    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut log_file = File::create(dvr_post_log)?;

    writeln!(log_file, "'{}' Plex DVR Postprocessing script started", time)?;

    // Check if post-processing is already running
    while fs::metadata(lock_file).is_ok() {
        writeln!(
            log_file,
            "'{}' '{}' exists, sleeping processing of '{}'",
            time, lock_file, in_file
        )?;
        sleep(Duration::from_secs(10));
    }

    // Create lock file to prevent other post-processing from running simultaneously
    writeln!(
        log_file,
        "'{}' Creating lock file for processing '{}'",
        time, in_file
    )?;
    File::create(lock_file)?;
    cargo install --path . --locked
    // Encode file to MP4 with HandBrakeCLI
    writeln!(log_file, "'{}' Transcoding started on '{}'", time, in_file)?;
    let status = Command::new(handbrake)
        .args(&[
            "-i",
            &in_file,
            "-o",
            &tmp_file,
            "--preset=Fast 1080p30",
            "--encoder=x265_10bit",
            "--encopts=rate_control=CBR",
            "-O",
        ])
        .status();

    if let Err(e) = status {
        writeln!(log_file, "Error during transcoding: {}", e)?;
        fs::remove_file(lock_file)?;
        exit(1);
    }

    // Overwrite original ts file with the transcoded file
    writeln!(log_file, "'{}' File rename started", time)?;
    fs::rename(&tmp_file, &in_file)?;

    // Remove lock file
    writeln!(log_file, "'{}' All done! Removing lock for '{}'", time, in_file)?;
    fs::remove_file(lock_file)?;

    Ok(())
}