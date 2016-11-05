extern crate rust_sense_hat;

use rust_sense_hat::LedDisplay;
use rust_sense_hat::color::Color;
use std::{thread, time};

fn main() {
    let mut display = LedDisplay::new().unwrap();
    display.clear();

    let t = time::Duration::from_millis(150);
    let white = Color::Hex888(0xFFFFFF);
    let red = Color::Hex888(0xFF0000);
    let blue = Color::Rgb(0, 0, 150);
    let green = Color::Rgb(0, 150, 0);
    let done = time::Duration::from_millis(500);

    for i in 0..8 {
        for j in 0..8 {
            display.draw_pixel(i, j, white);
        }
        thread::sleep(t);
    }

    thread::sleep(done);

    for i in 0..8 {
        for j in 0..8 {
            display.draw_pixel(j, i, red);
        }
        thread::sleep(t);
    }

    thread::sleep(done);

    for i in (0..8).rev() {
        for j in 0..8 { 
            display.draw_pixel(i, j, blue);
        }
        thread::sleep(t);
    }

    thread::sleep(done);

    for i in (0..8).rev() {
        for j in (0..8).rev() {
            display.draw_pixel(j, i, green);
        }
        thread::sleep(t);
    }

    thread::sleep(done);
    display.clear();
}
