use fltk::{app, enums::Color, frame::Frame, prelude::*, window::Window};
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use winapi::um::winuser::{SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW};

fn main() {
    let app = app::App::default();
    
    // Create a window with a specified position (1000, 0) and size (100x100)
    let mut window = Window::default()
        .with_size(100, 100)
        .with_pos(1000, 0)
        .with_label("");

    // Set the background color of the window to a specific color
    // This color (0xabcdef) will be used to simulate transparency
    window.set_color(Color::from_u32(0xabcdef));

    // Set the window's frame type to FlatBox
    // This removes any additional styling or borders around the window
    window.set_frame(fltk::enums::FrameType::FlatBox);

    // Remove the window border to make it look like a standalone box
    window.set_border(false);

    // Create a frame (essentially a rectangular area) within the window
    // The frame's position is (0, 0) and its size is (100x100)
    let mut frame = Frame::default()
        .with_size(100, 100)
        .with_pos(0, 0)
        .with_label("");

    // Set the color of the frame to red
    frame.set_color(Color::Red);

    // Set the frame type to FlatBox for the frame as well
    frame.set_frame(fltk::enums::FrameType::FlatBox);
    
    // End the window setup
    // This tells FLTK that we are done adding widgets to the window
    window.end();

    // Show the window on the screen
    window.show();
    
    // Use the raw_handle to get the window handle and set it to be always on top
    let raw_handle = window.raw_handle();
    if !raw_handle.is_null() {
        unsafe {
            SetWindowPos(
                raw_handle as *mut _,
                HWND_TOPMOST,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
            );
        }
    }
    
    // Variables to store the initial mouse coordinates when dragging starts
    let mut initial_x = 0;
    let mut initial_y = 0;

    // Handle window events, particularly for dragging and keyboard input
    window.handle(move |win, event| match event {
        fltk::enums::Event::Push => {
            // Get the current mouse coordinates
            let (x, y) = app::event_coords();

            // Store the initial coordinates for calculating the drag offset
            initial_x = x;
            initial_y = y;
            true
        }
        fltk::enums::Event::Drag => {
            // Get the current mouse coordinates
            let (x, y) = app::event_coords();

            // Calculate the new position of the window based on the drag offset
            win.set_pos(win.x() + x - initial_x, win.y() + y - initial_y);
            true
        }
        _ => false,
    });

    let window_arc = Arc::new(Mutex::new(window.clone()));
    let window_arc_clone = Arc::clone(&window_arc);
    let target_pos = Arc::new(Mutex::new((1000, 0))); // Initialize with the initial position
    let target_pos_clone = Arc::clone(&target_pos);
    let is_moving = Arc::new(Mutex::new(false));
    let is_moving_clone = Arc::clone(&is_moving);

    // Thread to update the target position every 10 seconds
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(10));
        let mut rng = rand::thread_rng();
        let screen_width = app::screen_size().0 as i32;
        let screen_height = app::screen_size().1 as i32;
        let new_x = rng.gen_range(0..screen_width - 100);
        let new_y = rng.gen_range(0..screen_height - 100);
        {
            let mut target = target_pos_clone.lock().unwrap();
            target.0 = new_x;
            target.1 = new_y;
        }
        {
            let mut moving = is_moving_clone.lock().unwrap();
            *moving = true;
        }
    });

    // Idle function to smoothly move the window
    app::add_idle3(move |_| {
        let mut window = window_arc.lock().unwrap();
        let mut moving = is_moving.lock().unwrap();
        if *moving {
            let target = target_pos.lock().unwrap();
            let target_x = target.0;
            let target_y = target.1;

            let current_x = window.x();
            let current_y = window.y();

            let step = 1; // Reduce the step size for slower movement
            let new_x = if (target_x - current_x).abs() <= step {
                target_x
            } else {
                current_x + (target_x - current_x).signum() * step
            };
            let new_y = if (target_y - current_y).abs() <= step {
                target_y
            } else {
                current_y + (target_y - current_y).signum() * step
            };
            window.set_pos(new_x, new_y);

            if new_x == target_x && new_y == target_y {
                *moving = false;
            }
        }
    });

    // Run the FLTK application
    // This starts the event loop, making the window responsive to user interactions
    app.run().unwrap();
}
