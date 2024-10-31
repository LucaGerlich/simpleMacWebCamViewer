use opencv::{highgui, prelude::*, videoio, Result};
use std::io::{self, Write};

fn main() -> Result<()> {
    // Define the range of camera indices to check
    const MAX_CAMERAS: i32 = 10;
    let mut available_cameras = Vec::new();

    println!("Detecting available cameras...");

    // Iterate through possible camera indices and check availability
    for index in 0..MAX_CAMERAS {
        let cam = videoio::VideoCapture::new(index, videoio::CAP_ANY)?;
        if videoio::VideoCapture::is_opened(&cam)? {
            available_cameras.push(index);
            println!("Camera {} is available.", index);
            // Release the camera
            drop(cam);
        }
    }

    if available_cameras.is_empty() {
        eprintln!("No cameras found. Exiting.");
        return Ok(());
    }

    // Determine which camera to use
    let selected_camera = if available_cameras.len() == 1 {
        println!(
            "Only one camera detected. Using Camera {}.",
            available_cameras[0]
        );
        available_cameras[0]
    } else {
        println!("\nMultiple cameras detected:");
        for (i, &cam_index) in available_cameras.iter().enumerate() {
            println!("  {}: Camera {}", i, cam_index);
        }

        // Prompt the user to select a camera
        loop {
            print!("\nEnter the number corresponding to the camera you want to use: ");
            io::stdout().flush().unwrap(); // Ensure the prompt is displayed

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            // Attempt to parse the input to a usize index
            match input.trim().parse::<usize>() {
                Ok(num) if num < available_cameras.len() => {
                    let cam_index = available_cameras[num];
                    println!("Selected Camera {}.", cam_index);
                    break cam_index;
                }
                _ => {
                    println!("Invalid selection. Please enter a valid number.");
                }
            }
        }
    };

    // Initialize the video capture with the selected camera
    let mut cam = videoio::VideoCapture::new(selected_camera, videoio::CAP_ANY)?;
    if !videoio::VideoCapture::is_opened(&cam)? {
        eprintln!("Unable to open Camera {}.", selected_camera);
        return Ok(());
    }

    // Optionally, set the camera resolution (uncomment if needed)
    // cam.set(videoio::CAP_PROP_FRAME_WIDTH, 1280.0)?;
    // cam.set(videoio::CAP_PROP_FRAME_HEIGHT, 720.0)?;
    cam.set(videoio::CAP_PROP_FPS, 120.0)?;

    // Create a window
    let window_name = "Webcam";
    highgui::named_window(window_name, highgui::WINDOW_AUTOSIZE)?;

    println!("\nPress 'q' or 'Esc' to exit.");

    loop {
        let mut frame = Mat::default();
        // Capture a frame
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            highgui::imshow(window_name, &frame)?;
        }

        // Wait for 10 ms and check if the user pressed the 'q' key or 'Esc' to quit
        let key = highgui::wait_key(10)?;
        if key == 'q' as i32 || key == 27 {
            println!("Exiting...");
            break;
        }
    }

    Ok(())
}
