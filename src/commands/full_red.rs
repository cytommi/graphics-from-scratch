use crate::canvas::{colors, Canvas};
use crate::Command;

pub struct RedScreen;

const RED: colors::Rgb = colors::Rgb {
    red: 255.0,
    green: 0.0,
    blue: 0.0,
};

impl Command for RedScreen {
    fn run(canvas: &mut Canvas) {
        let width = canvas.dimensions.width;
        let height = canvas.dimensions.height;

        for i in 0..(width / 2) as i32 {
            for j in 0..(height / 2) as i32 {
                canvas.put_pixel(i, j, &RED);
                canvas.put_pixel(i, -j, &RED);
                canvas.put_pixel(-i, j, &RED);
                canvas.put_pixel(-i, -j, &RED);
            }
        }

        canvas.display_until_exit();
    }
}
