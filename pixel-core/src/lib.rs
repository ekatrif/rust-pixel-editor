use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl Canvas {
    pub const MAX_DIMENSION: u32 = 256;

    pub fn new(width: u32, height: u32) -> Result<Self, &'static str> {
        if width == 0 || height == 0 || width > Self::MAX_DIMENSION || height > Self::MAX_DIMENSION
        {
            return Err("canvas dimensions must be between 1 and 256 pixels");
        }

        let byte_count = width as usize * height as usize * 4;
        let pixels = vec![0; byte_count];
        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let offset = (y as usize * self.width as usize + x as usize) * 4;
        Some(Color::new(
            self.pixels[offset],
            self.pixels[offset + 1],
            self.pixels[offset + 2],
            self.pixels[offset + 3],
        ))
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let offset = (y as usize * self.width as usize + x as usize) * 4;
        self.pixels[offset..offset + 4].copy_from_slice(&[color.r, color.g, color.b, color.a]);
        true
    }

    pub fn clear(&mut self, color: Color) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(x, y, color);
            }
        }
    }
}

#[wasm_bindgen]
pub fn canvas_area(width: u32, height: u32) -> u32 {
    width * height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_area_test() {
        let result = canvas_area(32, 32);
        assert_eq!(result, 1024);

        let result2 = canvas_area(1, 7);
        assert_eq!(result2, 7);
    }

    #[test]
    fn color_test() {
        let red = Color::new(255, 0, 0, 255);
        let transparent = Color::TRANSPARENT;
        assert_eq!(red.r, 255);
        assert_ne!(red, transparent);
    }

    #[test]
    fn canvas_new_creates_transparent_rgba_buffer() {
        let canvas = Canvas::new(32, 32).expect("32x32 is a valid canvas size");

        assert_eq!(canvas.width, 32);
        assert_eq!(canvas.height, 32);
        assert_eq!(canvas.pixels.len(), 32 * 32 * 4);
        assert!(canvas.pixels.iter().all(|&channel| channel == 0));
    }

    #[test]
    fn canvas_new_rejects_invalid_dimensions() {
        assert!(Canvas::new(0, 32).is_err());
        assert!(Canvas::new(32, 0).is_err());
        assert!(Canvas::new(257, 32).is_err());
        assert!(Canvas::new(32, 257).is_err());
    }

    #[test]
    fn get_pixel_reads_the_right_position_and_checks_bounds() {
        let mut canvas = Canvas::new(2, 2).expect("2x2 is a valid canvas size");
        canvas.pixels[12..16].copy_from_slice(&[255, 10, 20, 128]);

        assert_eq!(canvas.get_pixel(0, 0), Some(Color::TRANSPARENT));
        assert_eq!(canvas.get_pixel(1, 1), Some(Color::new(255, 10, 20, 128)));
        assert_eq!(canvas.get_pixel(2, 1), None);
        assert_eq!(canvas.get_pixel(1, 2), None);
    }

    #[test]
    fn set_pixel_updates_only_an_in_bounds_pixel() {
        let mut canvas = Canvas::new(2, 2).expect("2x2 is a valid canvas size");
        let red = Color::new(255, 0, 0, 255);

        assert!(canvas.set_pixel(1, 1, red));
        assert_eq!(canvas.get_pixel(1, 1), Some(red));
        assert_eq!(canvas.get_pixel(0, 1), Some(Color::TRANSPARENT));

        assert!(!canvas.set_pixel(2, 1, red));
        assert_eq!(canvas.get_pixel(1, 1), Some(red));
    }

    #[test]
    fn clear_fills_every_pixel_with_selected_color() {
        let mut canvas = Canvas::new(2, 2).expect("2x2 is a valid canvas size");
        let red = Color::new(255, 0, 0, 255);
        let blue = Color::new(0, 0, 255, 255);
        canvas.set_pixel(0, 0, red);
        canvas.set_pixel(1, 1, blue);

        canvas.clear(red);
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(canvas.get_pixel(x, y), Some(red));
            }
        }

        canvas.clear(Color::TRANSPARENT);
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(canvas.get_pixel(x, y), Some(Color::TRANSPARENT));
            }
        }
    }
}
