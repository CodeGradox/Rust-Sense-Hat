extern crate framebuffer;
extern crate glob;

use framebuffer::Framebuffer;
use glob::glob;

pub enum Color {
    Rgb(u8, u8, u8),
    Hex565(u16),
    Hex888(u32),
}

impl Color {
    // Converts a color to a 16 bit rgb565 color and splits
    // it into two u8.
    fn split(&self) -> (u8, u8) {
        match *self {
            Color::Rgb(r, g, b) => {
                let r = (r >> 3) << 3;
                let g = g >> 2;
                let b = b >> 3;
                (r | (g >> 3), b | (g << 5))
            },
            Color::Hex565(h) => ((h >> 8) as u8, h as u8),
            Color::Hex888(h) => {
                let r = (h >> 19) & 0x1F;
                let g = (h >> 10) & 0x3F;
                let b = (h >> 3) & 0x1F;
                let rgb = (r << 11) + (g << 5) + b;
                ((rgb >> 8) as u8, rgb as u8) 
            },
        }
    }
}

pub struct LedDisplay {
    framebuffer: Framebuffer,
    frame: Vec<u8>,
}

impl LedDisplay {
    pub fn new() -> Result<Self, ()> { 
        // temporary framebuffer
        let mut fb_tmp: Option<Framebuffer> = None;
        // Id for the Sense Hat framebuffer
        let rpi_sense_fb = b"RPi-Sense FB";
        
        // Check if any displays are connected
        let path = match glob("/dev/fb*") {
            Ok(p) => p,
            Err(_) => return Err(()),
        };  

        // Check every file buffer and see if it is
        // the fb for the Sense Hat LED display
        for entry in path {
            if let Ok(file_path) = entry {
                if let Ok(fb) = Framebuffer::new(&file_path.to_string_lossy()) {
                    let id = fb.fix_screen_info.id;
                    if rpi_sense_fb == &id[..rpi_sense_fb.len()] {
                        fb_tmp = Some(fb);
                        break;
                    }
                }
            }
        }

        let framebuffer = match fb_tmp {
            Some(fb) => fb,
            None => {
                println!("Could not find fb device");
                return Err(())
            },
        };
       
        let line_length = framebuffer.fix_screen_info.line_length as usize;

        Ok(Self {
            framebuffer: framebuffer,
            frame: vec![0u8; line_length * 8],
        })
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) { 
        assert!(x <= 7, "X position must be within 0 and 7");
        assert!(y <= 7, "Y position must be within 0 and 7");
        
        let (msb, lsb) = color.split();
        // The position of the pixel. One pixel is u16
        // but is stoed as two u8.
        let pos = 2*(x + y*8);

        // Each pixel is stored in little endian, so we need to flip
        // the two values as they are big endian.
        self.frame[pos] = lsb;
        self.frame[pos + 1] = msb;
        self.framebuffer.write_frame(&self.frame);
    }

    pub fn clear(&mut self) {
        for val in self.frame.iter_mut() {
            *val = 0u8;
        }
        self.framebuffer.write_frame(&self.frame);
    }
}

