use raylib::prelude::*;

pub struct FrameBuffer {
    pixels: Vec<Color>,
    width: i32,
    height: i32,
}

impl FrameBuffer {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            pixels: vec![Color::BLACK; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            self.pixels[(y * self.width + x) as usize] = color;
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Option<Color> {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            Some(self.pixels[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}
