use std::fmt;
use std::str::FromStr;
#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Default for Color {
    fn default() -> Self {
        Color::new(0, 255, 0, 34)
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    pub fn fmt_ass(&self) -> String {
        format!(
            "&H{:0>2X}{:0>2X}{:0>2X}{:0>2X}",
            self.a, self.b, self.g, self.r
        )
    }
    pub fn fmt_ssa(&self) -> String {
        format!("&H{:0>2X}{:0>2X}{:0>2X}", self.b, self.g, self.r)
    }
}

impl FromStr for Color {
    type Err = std::num::ParseIntError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str.len() == 5 {
            Ok(Color {
                r: u8::from_str_radix(&str[2..4], 16)?,
                g: 0,
                b: 0,
                a: 0,
            })
        } else if str.len() == 7 {
            Ok(Color {
                r: u8::from_str_radix(&str[4..6], 16)?,
                g: u8::from_str_radix(&str[2..4], 16)?,
                b: 0,
                a: 0,
            })
        } else if str.len() == 9 {
            Ok(Color {
                a: u8::from_str_radix(&str[2..4], 16)?,
                b: u8::from_str_radix(&str[4..6], 16)?,
                g: u8::from_str_radix(&str[6..8], 16)?,
                r: u8::from_str_radix(&str[8..10], 16)?,
            })
        } else {
            Ok(Color {
                a: u8::from_str_radix(&str[2..4], 16)?,
                b: u8::from_str_radix(&str[4..6], 16)?,
                g: u8::from_str_radix(&str[6..8], 16)?,
                r: u8::from_str_radix(&str[8..10], 16)?,
            })
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:x}{:x}{:x}", self.r, self.g, self.b)
    }
}
#[derive(Debug)]
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

impl Alignment {
    pub fn from_str(str: &str) -> Result<Self, &'static str> {
        match str {
            "1" => Ok(Alignment::BottomLeft),
            "2" => Ok(Alignment::BottomCenter),
            "3" => Ok(Alignment::BottomRight),
            "4" => Ok(Alignment::MiddleLeft),
            "5" => Ok(Alignment::MiddleCenter),
            "6" => Ok(Alignment::MiddleRight),
            "7" => Ok(Alignment::TopLeft),
            "8" => Ok(Alignment::TopCenter),
            "9" => Ok(Alignment::TopRight),
            &_ => Err("ParseIntError"),
        }
    }
}
