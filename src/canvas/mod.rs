pub mod colors;

use minifb::{Key, ScaleMode, Window, WindowOptions};

use colors::Rgb;

#[derive(Debug)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub struct Canvas {
    window: Window,
    buffer: Vec<u32>,
    pub dimensions: Dimensions,
}

impl Canvas {
    /// Creates a new window with a title of `name` and with the usable area (i.e., excluding
    /// window title bar and decorations), of the given `width` and `height`. The window is set
    /// as non-resizable to avoid the complexity of changing the usable canvas area in response
    /// to user actions.
    pub fn new(name: &str, width: usize, height: usize) -> Self {
        let mut window = Window::new(
            name,
            width,
            height,
            WindowOptions {
                resize: false,
                scale_mode: ScaleMode::UpperLeft,
                ..WindowOptions::default()
            },
        )
        .expect("Window creation failed");

        // Limit frame rate to a maximum of 50 frames per second to reduce CPU usage
        window.limit_update_rate(Some(std::time::Duration::from_micros(20_000)));

        let mut buffer: Vec<u32> = Vec::with_capacity(width * height);
        buffer.resize(width * height, 0);
        Canvas {
            window,
            buffer,
            dimensions: Dimensions { width, height },
        }
    }

    /// Clears the canvas with `color`.
    pub fn clear_canvas(&mut self, color: &Rgb) {
        let col: u32 =
            (color.red as u32) * 65536 + (color.green as u32) * 256 + (color.blue as u32);

        for i in 0..self.buffer.len() {
            self.buffer[i] = col;
        }
    }

    /// Sets a single pixel on the canvas at the given `x`, `y` coordinates. The center of the canvas is the origin,
    /// i.e., where `x = 0, y = 0`.
    /// `x` is the horizontal component ranging from `-width/2` at the furthest left to `width/2 - 1` at the
    /// furthest right.
    /// `y` is the vertical component, ranging from `-height/2` at the bottom to `height/2 - 1` at the top.
    /// If either `x` or `y` is outside these ranges, the function returns without setting a pixel.
    /// The pixel `color` is an [`Rgb` struct](struct.Rgb.html) defining red, green and blue components.
    /// Changes will only become visible when [`display_until_exit`](#method.display_until_exit) is called.
    pub fn put_pixel(&mut self, x: i32, y: i32, color: &Rgb) {
        let (width, height) = (self.dimensions.width as i32, self.dimensions.height as i32);

        let screen_x = width / 2 + x;
        let screen_y = height / 2 - y - 1;

        if (screen_x < 0) | (screen_x >= width) | (screen_y < 0) | (screen_y >= height) {
            return;
        }

        let pixel_pos_in_buffer = (screen_x + width * screen_y) as usize;

        self.buffer[pixel_pos_in_buffer] =
            (color.red as u32) * 65536 + (color.green as u32) * 256 + (color.blue as u32);
    }

    ///  Updates the window with all pixels set using `put_pixel` and displays this by looping continuously until
    ///  the window closes or the user presses the escape key. This function does not return until either event
    ///  occurs, which means this function can only be called once in a program.
    pub fn display_until_exit(&mut self) {
        // The unwrap causes the code to exit if the update fails
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window
                .update_with_buffer(&self.buffer, self.dimensions.width, self.dimensions.height)
                .unwrap();
        }
    }
}
