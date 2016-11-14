pub mod color;

extern crate framebuffer;
extern crate glob;

use framebuffer::Framebuffer;
use glob::glob;

use std::error::Error;
use std::fmt;

use color::Color;

pub struct LedDisplay {
    framebuffer: Framebuffer,
}

impl LedDisplay {
    pub fn new() -> Result<Self, LedDisplayError> {
        // Id for the Sense Hat framebuffer
        let rpi_sense_fb = b"RPi-Sense FB";

        // Check if any displays are connected
        let path = match glob("/dev/fb*") {
            Ok(p) => p,
            Err(_) => {
                return Err(LedDisplayError::new(LedDisplayErrorKind::IoError,
                                                "Could not find any framebuffers.Please connect \
                                                 your Sense Hat to the Raspberry Pi."))
            }
        };

        // Try to find the Sense Hat frame buffer
        let framebuffer = path.filter_map(Result::ok)
            .filter_map(|file_path| Framebuffer::new(&file_path.to_string_lossy()).ok())
            .filter(|fb| {
                let id = fb.fix_screen_info.id;
                rpi_sense_fb == &id[..rpi_sense_fb.len()]
            })
            .next();

        match framebuffer {
            Some(fb) => Ok(Self { framebuffer: fb }),
            None => {
                Err(LedDisplayError::new(LedDisplayErrorKind::IoError,
                                         "Cannot detect RPi-Sense FB device"))
            }
        }
    }

    // Paints the whole LED with a signle color
    pub fn set_frame(&mut self, color: Color) {
        let (msb, lsb) = color.split();
        let mut frame = self.get_frame();
        for i in (0..64).map(|x| x * 2) {
            frame[i] = lsb;
            frame[i + 1] = msb;
        }
    }

    // Draws one pixel on the LED display
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        assert!(x <= 7, "X position must be within 0 and 7");
        assert!(y <= 7, "Y position must be within 0 and 7");

        let (msb, lsb) = color.split();
        let pos = 2 * (x + y * 8);
        let mut frame = self.get_frame();
        // Each pixel is stored in little endian, so we need to flip
        // the two values as they are big endian.
        frame[pos] = lsb;
        frame[pos + 1] = msb;
    }

    pub fn set_pixels(&mut self, color_list: &[Color]) {
        let mut frame = self.get_frame();
        for (i, c) in (0..64).map(|x| x * 2).zip(color_list.iter()) {
            let (msb, lsb) = c.split();
            frame[i] = lsb;
            frame[i + 1] = msb;
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.write_frame(&[0u8; 128]);
    }

    fn get_frame(&mut self) -> &mut [u8] {
        return unsafe { self.framebuffer.frame.as_mut_slice() };
    }
}

#[derive(Debug)]
pub enum LedDisplayErrorKind {
    IoError,
}

#[derive(Debug)]
pub struct LedDisplayError {
    pub kind: LedDisplayErrorKind,
    pub details: String,
}

impl LedDisplayError {
    fn new(kind: LedDisplayErrorKind, details: &str) -> Self {
        Self {
            kind: kind,
            details: details.to_string(),
        }
    }
}

impl std::error::Error for LedDisplayError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for LedDisplayError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description())
    }
}
