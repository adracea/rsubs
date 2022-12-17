use std::fmt;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Default for Color {
    fn default() -> Self {
        Color::new(0, 0, 0, 0)
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:x}{:x}{:x}", self.r, self.g, self.b)
    }
}

pub enum Alignment {
    BottomLeft = 1,
    BottomCenter = 2,
    BottomRight = 3,
    MiddleLeft = 4,
    MiddleCenter = 5,
    MiddleRight = 6,
    TopLeft = 7,
    TopCenter = 8,
    TopRight = 9,
}
