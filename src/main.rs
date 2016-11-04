extern crate rust_sense_hat;

use rust_sense_hat::{LedDisplay, Color};

fn main() {
    //use Color::{Rgb, Hex565, Hex888};
    let mut display = LedDisplay::new().unwrap();
    display.clear();

    display.draw_pixel(0, 0, Color::Hex888(0xafcfdb));
    display.draw_pixel(0, 2, Color::Hex565(0xae7b));
    display.draw_pixel(0, 4, Color::Rgb(175, 207, 219));
    display.clear();
}

