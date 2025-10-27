use opencv::{
    core::Vector, // Import Vector directly from core
    imgcodecs,
    prelude::*, // Basic traits
    videoio,    // Video I/O
};
use std::{
    error::Error,
    fs, // File system operations (creating directory)
    path::Path,
    thread, // For sleeping
    time::{Duration, Instant},
};
use chrono::Local; // Add chrono back for date

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting RTSP Frame Grabber...");

    // --- Configuration ---
    let rtsp_url = "rtsp://admin:Spiderman1215@192.168.137.192:5543/live/channel0";
    let output_dir = "captured_frames";
    let frame_interval_ms: u64 = 60000; // Capture one frame every 60000ms (1 minute)
                                       // --- ---

    // Create the output directory if it doesn't exist
    if !Path::new(output_dir).exists() {
        println!("Creating output directory: {}", output_dir);
        fs::create_dir_all(output_dir)?;
    } else {
        println!("Output directory already exists: {}", output_dir);
    }

    println!("Connecting to RTSP stream: {}", rtsp_url);
    // Try opening the stream using FFMPEG backend
    let mut cam = videoio::VideoCapture::from_file(rtsp_url, videoio::CAP_FFMPEG)?;
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        eprintln!("Error: Unable to open video stream.");
        eprintln!("Check the RTSP URL, camera connection, network, and firewall.");
        return Err("Failed to open video capture".into());
    }
    println!("Stream connected successfully.");

    let target_duration = Duration::from_millis(frame_interval_ms);
    let mut frame_count = 0; // Counter for saved frames

    loop {
        let loop_start = Instant::now();
        let mut frame = Mat::default(); // Create an empty Mat object

        // Read a frame from the camera
        match cam.read(&mut frame) {
            Ok(true) => {
                if frame.empty() {
                    println!("Warning: Received empty frame. Skipping.");
                    continue; // Skip if frame is empty
                }

                // *** MODIFIED FILENAME GENERATION ***
                // Get current date as YYYYMMDD
                let current_date = Local::now().format("%Y%m%d");
                // Generate filename with sequential number and date
                let filename = format!("{}/{}_{}.jpg", output_dir, frame_count + 1, current_date);

                // Save the frame to the specified file
                // *** Use imported Vector name ***
                let params = Vector::<i32>::new(); // Use default parameters for imwrite
                match imgcodecs::imwrite(&filename, &frame, &params) {
                    Ok(true) => {
                        frame_count += 1; // Increment counter *after* successful save
                        println!("Frame {} saved: {}", frame_count, filename);
                    }
                    Ok(false) => {
                        eprintln!("Error: Failed to save frame to {}", filename);
                        // Potentially permissions issue or invalid path component
                    }
                    Err(e) => {
                        // Don't increment frame_count on error
                        eprintln!("Error saving frame {} to {}: {}", frame_count + 1, filename, e);
                    }
                }
            }
            Ok(false) => {
                println!("End of video stream reached or cannot read frame. Exiting.");
                break; // Exit loop if stream ends or read fails consistently
            }
            Err(e) => {
                eprintln!("Error reading frame: {}. Exiting.", e);
                break; // Exit loop on critical read error
            }
        }

        // Calculate how long to sleep to maintain the desired frame interval
        let elapsed = loop_start.elapsed();
        if elapsed < target_duration {
            let sleep_duration = target_duration - elapsed;
            thread::sleep(sleep_duration);
        } else {
            println!(
                "Warning: Frame processing took longer than the interval: {:?}",
                elapsed
            );
            // Optionally add a minimal sleep to prevent busy-waiting if processing is too long
            thread::sleep(Duration::from_millis(10)); // Prevent 100% CPU usage
        }
    }

    println!("Frame grabbing stopped. Total frames saved: {}", frame_count);
    // VideoCapture is automatically released when 'cam' goes out of scope

    Ok(())
}

