use once_cell::sync::Lazy;
use regex::Regex;
use std::result::Result;
use std::{fmt, ops::Div};

pub static TIMESTAMP: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\d{1,2}):(\d{1,2}):(\d{1,2})[.,](\d{1,3})").expect("Timestamp regex failure")
});

pub static TIMESTAMP_SHORT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\d{1,2}):(\d{2}):(\d{2})").expect("Short timestamp regex failure"));
pub fn make_time(
    h: Option<String>,
    m: Option<String>,
    s: Option<String>,
    ms: Option<String>,
    frames: Option<String>,
    fps: Option<String>,
) -> Time {
    let h2 = h.or(Some("0".to_string())).unwrap();
    let m2 = m.or(Some("0".to_string())).unwrap();
    let s2 = s.or(Some("0".to_string())).unwrap();
    let ms2 = ms.or(Some("0".to_string())).unwrap();
    let f2 = frames.or(Some("0".to_string()));
    let fs2 = fps.or(Some("0".to_string()));
    Time {
        h: h2,
        m: m2,
        s: s2,
        ms: ms2,
        frames: f2,
        fps: fs2,
    }
}
pub struct Time {
    h: String,
    m: String,
    s: String,
    ms: String,
    frames: Option<String>,
    fps: Option<String>,
}

impl Default for Time {
    fn default() -> Self {
        Time::new(
            0.to_string(),
            0.to_string(),
            0.to_string(),
            0.to_string(),
            None,
            None,
        )
    }
}

pub fn frames_to_ms(frames: u32, fps: f32) -> u32 {
    if frames == 0 || fps == 0.0 {
        0
    } else {
        (((frames as f32) * (10000.0)) / fps).round() as u32 / 10
    }
}
pub fn ms_to_frames(ms: u32, fps: f32) -> u32 {
    if fps == 0.0 {
        0
    } else {
        ((ms as f32) * fps / 1000.0).round() as u32
    }
}
pub fn ms_to_time(ms: u32) -> String {
    let hms = ms.div(3600000);
    let mms = (ms - hms * 3600000).div(60000);
    let sms = (ms - mms * 60000 - hms * 3600000).div(1000);
    let msms = &format!(
        "{}",
        ((ms - mms * 60000 - hms * 3600000 - sms * 1000) as f32).div(1000.0)
    )
    .split('.')
    .collect::<Vec<&str>>()
    .get(1)
    .or_else(|| Some(&"0"))
    .expect("Should be good")
    .to_string();
    // println!(
    //     "{}",
    //     format!("{:0>2}", hms)
    //         + ":"
    //         + &format!("{:0>2}", mms).to_string()
    //         + ":"
    //         + &format!("{:0>2}", sms).to_string()
    //         + "."
    //         + msms
    // );
    format!("{:0>2}", hms)
        + ":"
        + &format!("{:0>2}", mms).to_string()
        + ":"
        + &format!("{:0>2}", sms).to_string()
        + "."
        + msms
}
pub fn time_from_string(s: String) -> Time {
    let mut t = Time::default();
    t.h = s.split(':').collect::<Vec<&str>>()[0].to_string();
    t.m = s.split(':').collect::<Vec<&str>>()[1].to_string();
    t.s = s.split(':').collect::<Vec<&str>>()[2]
        .to_string()
        .split(&[',', '.'])
        .collect::<Vec<&str>>()[0]
        .to_string();
    t.ms = s
        .split(':')
        .collect::<Vec<&str>>()
        .get(2)
        .unwrap_or(&"0")
        .to_string()
        .split(&[',', '.'])
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap_or(&"0")
        .to_string();
    t
}
impl Time {
    pub fn new(
        h: String,
        m: String,
        s: String,
        ms: String,
        frames: Option<String>,
        fps: Option<String>,
    ) -> Time {
        Time {
            h,
            m,
            s,
            ms,
            frames,
            fps,
        }
    }
    pub fn total_ms(&self) -> u32 {
        self.h.parse::<u32>().expect("Not an int") * 3600000
            + self.m.parse::<u32>().expect("Not an int") * 60000
            + self.s.parse::<u32>().expect("Not an int") * 1000
            + self.ms.parse::<u32>().expect("Not an int")
    }
    pub fn update_from_fps_frames(&mut self) {
        let t = time_from_string(ms_to_time(frames_to_ms(self.frames(), self.fps())));
        self.h = t.h;
        self.m = t.m;
        self.s = t.s;
        self.ms = t.ms;
    }
    pub fn timestamp_to_ms(&self) -> u32 {
        (self.h.parse::<u32>().expect("Not an int") * 3600000)
            + (self.m.parse::<u32>().expect("Not an int") * 60000)
            + ((self.s.to_string() + &".".to_string() + &self.ms)
                .parse::<f32>()
                .expect("Not an int")
                * 1000.0)
                .round() as u32
    }
    pub fn derive_frames(&mut self) {
        self.frames = Some(ms_to_frames(self.timestamp_to_ms(), self.fps()).to_string());
    }
    pub fn frames(&self) -> u32 {
        self.frames
            .clone()
            .unwrap_or_else(|| "0".to_string())
            .parse::<u32>()
            .expect("msg")
    }
    pub fn fps(&self) -> f32 {
        self.fps
            .clone()
            .unwrap_or_else(|| "0".to_string())
            .parse::<f32>()
            .expect("msg")
    }
    pub fn set_fps(&mut self, fps: f32) {
        self.fps = Some(fps.to_string());
    }
    pub fn update_from_ms(&mut self, ms: u32) {
        let t = time_from_string(ms_to_time(ms));
        self.h = t.h;
        self.m = t.m;
        self.s = t.s;
        self.ms = t.ms;
        self.derive_frames();
    }
    pub fn add_ms(&mut self, ms: u32) {
        self.update_from_ms(self.total_ms() + ms)
    }
    pub fn sub_ms(&mut self, ms: u32) -> Result<(), ()> {
        if ms > self.total_ms() {
            Err(())
        } else {
            self.update_from_ms(self.total_ms() - ms);
            Ok(())
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0>2}:{:0>2}:{:0>2}.{:0<3}",
            self.h, self.m, self.s, self.ms
        )
    }
}
