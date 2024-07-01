use fltk::{app, enums::Color, frame::Frame, prelude::*, window::Window};
use std::ptr;
use winapi::um::winuser::{SetWindowPos, HWND_TOPMOST, SWP_NOSIZE, SWP_NOMOVE, SWP_SHOWWINDOW};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(1000, 0, 100, 100, "");
    
    wind.set_color(Color::from_u32(0xabcdef));  // Transparent color
    wind.set_frame(fltk::enums::FrameType::FlatBox);
    wind.set_border(false); // Remove the window border

    let mut frame = Frame::new(0, 0, 100, 100, "");
    frame.set_color(Color::Red);
    frame.set_frame(fltk::enums::FrameType::FlatBox);
    
    wind.end();
    wind.show();
    
    // Use the raw_handle to get the window handle and set it to be always on top
    let raw_handle = wind.raw_handle();
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
    
    let mut dx = 0;
    let mut dy = 0;

    wind.handle(move |win, ev| match ev {
        fltk::enums::Event::Push => {
            let (x, y) = app::event_coords();
            dx = x;
            dy = y;
            true
        }
        fltk::enums::Event::Drag => {
            let (x, y) = app::event_coords();
            win.set_pos(win.x() + x - dx, win.y() + y - dy);
            true
        }
        _ => false,
    });
    
    app.run().unwrap();
}
