use regex::Regex;

pub mod subs;
pub mod util;
// pub enum Format {
//     ".srt" = "srt",
//     ".ass" = "ass",
//     ".ssa" = "ssa",
// }
pub struct WebVTT {
    time_stamp: Regex,
}
