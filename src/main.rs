extern crate rust_sense_hat;

use rust_sense_hat::LedDisplay;
use rust_sense_hat::color::Color::{Hex888, Hex565, Rgb};

fn main() {
    let mut display = LedDisplay::new().unwrap();
    display.clear();

    display.set_pixel(0, 0, Hex888(0xafcfdb));
    display.set_pixel(0, 2, Hex565(0xae7b));
    display.set_pixel(0, 4, Rgb(175, 207, 219));
    display.clear();
    draw_ok(&mut display);
    draw_bluetooth(&mut display);
    display.clear();
    // display.set_frame(Color::Rgb(255, 255, 255));
}

fn draw_ok(display: &mut LedDisplay) {
    let g = Rgb(0, 255, 0);
    let b = Rgb(0, 0, 0);
    let frame = &[
        g, g, g, b, g, b, b, g,
        g, b, g, b, g, b, g, b,
        g, b, g, b, g, g, b, b,
        g, b, g, b, g, b, g, b,
        g, g, g, b, g, b, b, g,
        b, b, b, b, b, b, b, b,
        b, b, b, b, b, b, b, b,
        b, b, b, b, b, b, b, b
    ];
    display.set_pixels(frame);
}

fn draw_bluetooth(display: &mut LedDisplay) {
    let c = Hex565(0xFFFF);
    let b = Rgb(0, 0, 255);
    let frame = &[
        b, b, b, b, b, c, c, c,
        c, b, c, c, b, c, c, c,
        c, c, b, c, b, c, c, c,
        c, c, c, b, b, b, b, b,
        c, b, b, b, b, c, c, b,
        c, c, c, b, c, b, c, b,
        c, c, c, b, c, c, b, b,
        c, c, c, c, c, c, c, b
    ];
    display.set_pixels(frame);
}
