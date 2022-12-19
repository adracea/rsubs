use core::panic;
use std::{borrow::Borrow, collections::HashMap, io::Read, str::FromStr};

use crate::util::{
    color::{self, Alignment, Color},
    time::{time_from_string, Time},
};
use once_cell::sync::Lazy;
use regex::Regex;
// use std::fmt;
#[derive(Debug)]
pub struct SSAStyle {
    pub name: String,
    pub fontname: String,
    pub fontsize: f32,
    pub firstcolor: color::Color,
    pub secondcolor: color::Color,
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

#[derive(Debug)]
pub struct SSAEvent {
    pub layer: i32,
    pub start: Time,
    pub end: Time,
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
            start: time_from_string("0:00:00.20".to_string()),
            end: time_from_string("0:00:02.20".to_string()),
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
    Lazy::new(|| Regex::new(r"\{[^}]*\}").expect("Timestamp regex failure"));
impl SSAEvent {
    pub fn is_comment(&self) -> bool {
        false
    }
}
#[derive(Debug)]
pub struct SSAFile {
    pub events: Vec<SSAEvent>,
    pub styles: Vec<SSAStyle>,
    pub info: HashMap<String, String>,
    pub format: String,
}
impl Default for SSAFile {
    fn default() -> Self {
        SSAFile {
            events: vec![SSAEvent::default()],
            styles: vec![SSAStyle::default()],
            info: HashMap::new(),
            format: ".ass".to_string(),
        }
    }
}

impl SSAFile {
    pub fn to_srt(&self) -> String {
        let mut a = "".to_string();
        let regex =
            Regex::new(r"(?P<main>\{\\(?P<type>.)(?P<trigger>.*?)\})").expect("Regex broke");
        for (i, j) in self.events.iter().enumerate() {
            a += &((i + 1).to_string()
                + "\r\n"
                + &j.start.to_string().replace('.', ",")
                + " --> "
                + &j.end.to_string().replace('.', ",")
                + "\r\n"
                + &j.text.replace("\\N", "\r\n")
                + "\r\n\r\n");

            for k in regex.captures_iter(&a.clone()) {
                if k.name("trigger").unwrap().as_str() == "0" {
                    a = a.replace(
                        k.name("main").unwrap().as_str(),
                        &("</".to_string() + k.name("type").unwrap().as_str() + ">"),
                    );
                } else if k.name("trigger").unwrap().as_str() == "1" {
                    a = a.replace(
                        k.name("main").unwrap().as_str(),
                        &("<".to_string() + k.name("type").unwrap().as_str() + ">"),
                    );
                }
            }
        }
        a
    }
}

pub fn parse(subpath: String) -> Result<SSAFile, std::io::Error> {
    use std::fs::File;
    let mut f = File::open(subpath)?;
    let mut b: String = "".to_string();
    let mut sub: SSAFile = SSAFile::default();
    f.read_to_string(&mut b)?;
    let c: Vec<&str> = b.split("\r\n\r\n").collect();
    for i in c {
        if i.contains("Styles]") {
            let mut style: HashMap<String, Vec<&str>> = HashMap::new();
            let keys = i
                .split("\r\n")
                .filter(|x| x.starts_with("Format:"))
                .collect::<String>();
            let fmtheaders = keys.strip_prefix("Format: ").unwrap().replace(' ', "");
            let finalheaders = fmtheaders.split(',').collect::<Vec<&str>>();
            style.insert("Format".to_string(), finalheaders);

            let keys2 = i
                .split("\n")
                .filter(|&x| x.starts_with("Style: "))
                .map(|x| {
                    x.strip_prefix("Style: ")
                        .unwrap()
                        .split(',')
                        .collect::<Vec<&str>>()
                        .get(0)
                        .unwrap()
                        .clone()
                })
                .collect::<Vec<&str>>();
            let values2 = i
                .split("\r\n")
                .filter(|&x| x.starts_with("Style: "))
                .map(|x| x.strip_prefix("Style: ").unwrap().borrow())
                .collect::<Vec<&str>>();
            for (i, j) in keys2.into_iter().enumerate() {
                style.insert(
                    j.to_string(),
                    values2.get(i).unwrap().split(',').collect::<Vec<&str>>(),
                );
            }
            for _ in (&style.clone().get(&"Format".to_string()).unwrap()).into_iter() {
                for (k, l) in style.clone().into_iter() {
                    if k == "Format".to_string() {
                        continue;
                    }
                    let mut styl = SSAStyle::default();
                    styl.name = l.get(0).expect("missing_name").to_string();
                    styl.fontname = l.get(1).expect("missing_name").to_string();
                    styl.fontsize = l
                        .get(2)
                        .expect("missing_name")
                        .to_string()
                        .parse::<f32>()
                        .expect("msg");
                    styl.firstcolor =
                        Color::from_str(l.get(3).expect("missing_name")).expect("msg");
                    styl.secondcolor =
                        Color::from_str(l.get(4).expect("missing_name")).expect("msg");
                    // styl.thirdcolor =
                    //     Color::from_str(l.get(5).expect("missing_name")).expect("msg");
                    styl.outlinecolor =
                        Color::from_str(l.get(5).expect("missing_name")).expect("msg");
                    styl.backgroundcolor =
                        Color::from_str(l.get(6).expect("missing_name")).expect("msg");
                    styl.bold = if l.get(7).expect("missing value") == &"-1" {
                        false
                    } else {
                        true
                    };
                    styl.italic = if l.get(8).expect("missing value") == &"-1" {
                        false
                    } else {
                        true
                    };
                    styl.unerline = if l.get(9).expect("missing value") == &"-1" {
                        false
                    } else {
                        true
                    };
                    styl.strikeout = if l.get(10).expect("missing value") == &"-1" {
                        false
                    } else {
                        true
                    };
                    styl.scalex = l
                        .get(11)
                        .expect("Not provided ScaleX")
                        .parse::<f32>()
                        .expect("ScaleX value not proper");
                    styl.scaley = l
                        .get(12)
                        .expect("Not provided ScaleY")
                        .parse::<f32>()
                        .expect("ScaleY value not proper");
                    styl.spacing = l
                        .get(13)
                        .expect("Not provided Spacing")
                        .parse::<f32>()
                        .expect("Spacing value not proper");
                    styl.angle = l
                        .get(14)
                        .expect("Not provided Spacing")
                        .parse::<f32>()
                        .expect("Spacing value not proper");
                    styl.borderstyle = l
                        .get(15)
                        .expect("Not provided borderstyle")
                        .parse::<i8>()
                        .expect("borderstyle value not proper");
                    styl.outline = l
                        .get(16)
                        .expect("Not provided Spacing")
                        .parse::<f32>()
                        .expect("Spacing value not proper");
                    styl.shadow = l
                        .get(17)
                        .expect("Not provided Spacing")
                        .parse::<f32>()
                        .expect("Spacing value not proper");
                    styl.alignment =
                        Alignment::from_str(l.get(18).expect("Not provided Spacing")).unwrap();
                    styl.lmargin = l
                        .get(19)
                        .expect("Not provided lmargin")
                        .parse::<i32>()
                        .expect("lmargin value not proper");
                    styl.rmargin = l
                        .get(20)
                        .expect("Not provided rmargin")
                        .parse::<i32>()
                        .expect("rmargin value not proper");
                    styl.vmargin = l
                        .get(21)
                        .expect("Not provided vmargin")
                        .parse::<i32>()
                        .expect("vmargin value not proper");
                    styl.alpha = 0;
                    styl.encoding = l
                        .get(22)
                        .expect("Not provided encoding")
                        .parse::<i32>()
                        .expect("encoding value not proper");
                    styl.drawing = false;
                    sub.styles.push(styl);
                }
            }
        }
        if i.contains("[Script Info]") {
            for j in i.split("\r\n").collect::<Vec<&str>>().iter() {
                let line = j.split_once(':').unwrap_or(("", ""));
                sub.info
                    .insert(line.0.to_string(), line.1.trim().to_string());
            }
            sub.info.remove("");
            if !sub.info.contains_key("ScaledBorderAndShadows") {
                sub.info
                    .insert("ScaledBorderAndShadows".to_string(), "yes".to_string());
            }
        }
        if i.contains("[Events]") {
            sub.events.remove(0);
            for j in i.split("\r\n") {
                if j.starts_with("Dialogue:") {
                    let mut ev = SSAEvent::default();
                    let line = j
                        .strip_prefix("Dialogue: ")
                        .unwrap()
                        .splitn(10, ',')
                        .collect::<Vec<&str>>();
                    ev.layer = line
                        .get(0)
                        .unwrap()
                        .parse::<i32>()
                        .expect("Failed to parse layer");
                    ev.start = time_from_string(line.get(1).unwrap().to_string());
                    ev.end = time_from_string(line.get(2).unwrap().to_string());
                    ev.style = line.get(3).unwrap().to_string();
                    ev.name = line.get(4).unwrap().to_string();
                    ev.lmargin = line
                        .get(5)
                        .unwrap()
                        .to_string()
                        .parse::<f32>()
                        .expect("couldn't conv to float");
                    ev.rmargin = line
                        .get(6)
                        .unwrap()
                        .to_string()
                        .parse::<f32>()
                        .expect("couldn't conv to float");
                    ev.vmargin = line
                        .get(7)
                        .unwrap()
                        .to_string()
                        .parse::<f32>()
                        .expect("couldn't conv to float");
                    ev.effect = line.get(8).unwrap().to_string();
                    ev.text = line.get(9).unwrap().to_string();
                    sub.events.push(ev);
                }
            }
        }
    }
    if true {
        Ok(sub)
    } else {
        panic!("test")
    }
}
