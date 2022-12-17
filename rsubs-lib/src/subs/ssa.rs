use crate::util::color::{self, Alignment, Color};
use once_cell::sync::Lazy;
use regex::Regex;
// use std::fmt;
pub struct SSAStyle {
    pub name: String,
    pub fontname: String,
    pub fontsize: f32,
    pub firstcolor: color::Color,
    pub secondcolor: color::Color,
    pub thirdcolor: color::Color,
    pub outlinecolor: color::Color,
    pub backgroundcolor: color::Color,
    pub bold: bool,
    pub italic: bool,
    pub unerline: bool,
    pub strikeout: bool,
    pub scalex: f32,
    pub scaley: f32,
    pub spacing: f32,
    pub angle: f32,
    pub borderstyle: i8,
    pub outline: f32,
    pub shadow: f32,
    pub alignment: color::Alignment,
    pub lmargin: i32,
    pub rmargin: i32,
    pub vmargin: i32,
    pub alpha: i32,
    pub encoding: i32,
    pub drawing: bool,
}

impl Default for SSAStyle {
    fn default() -> Self {
        SSAStyle {
            name: "Default".to_string(),
            fontname: "Arial".to_string(),
            fontsize: 20.0,
            firstcolor: Color::default(),
            secondcolor: Color::default(),
            thirdcolor: Color::default(),
            outlinecolor: Color::default(),
            backgroundcolor: Color::default(),
            bold: true,
            italic: false,
            unerline: false,
            strikeout: false,
            scalex: 25.5,
            scaley: 25.5,
            spacing: 0.0,
            angle: 0.0,
            borderstyle: 1,
            outline: 2.0,
            shadow: 2.0,
            alignment: Alignment::BottomCenter,
            lmargin: 10,
            rmargin: 10,
            vmargin: 10,
            alpha: 0,
            encoding: 1,
            drawing: false,
        }
    }
}

// impl SSAStyle {}
// impl std::fmt::Display for SSAStyle {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "<SSAStyle {}", self.h, self.m, self.s, self.ms)
//     }
// }
pub struct SSAEvent {
    pub layer: i32,
    pub start: String,
    pub end: String,
    pub style: String,
    pub name: String,
    pub lmargin: f32,
    pub rmargin: f32,
    pub vmargin: f32,
    pub effect: String,
    pub linetype: String,
    pub text: String,
}

impl Default for SSAEvent {
    fn default() -> Self {
        SSAEvent {
            layer: 0,
            start: "0:00:00.20".to_string(),
            end: "0:00:02.20".to_string(),
            style: "Default".to_string(),
            name: "".to_string(),
            lmargin: 0.0,
            rmargin: 0.0,
            vmargin: 0.0,
            effect: "".to_string(),
            linetype: "Dialogue".to_string(),
            text: "Lorem Ipsum".to_string(),
        }
    }
}

pub static OVERRIDE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"{[^}]*}").expect("Timestamp regex failure"));
impl SSAEvent {
    pub fn is_comment(&self) -> bool {}
}
