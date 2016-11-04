pub enum Color {
    Rgb(u8, u8, u8),
    Hex565(u16),
    Hex888(u32),
}

impl Color {
    // Converts a color to a 16 bit rgb565 color and splits
    // it into two u8.
    pub fn split(&self) -> (u8, u8) {
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


