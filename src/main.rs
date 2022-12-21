use std::fs::File;
use std::io::Read;

use rsubs_lib::subs::srt;
use rsubs_lib::subs::ssa;
use rsubs_lib::subs::ssa::SSAFile;
use rsubs_lib::subs::vtt;
use rsubs_lib::subs::vtt::VTTFile;

fn test_ssa_from_file_to_srt_file() {
    let ssafile = ssa::parse("I'm Home-E27.ass".to_string()).unwrap();
    ssafile
        .to_srt()
        .to_file("I'm Home-E27_1.srt".to_string())
        .expect("Couldn't write");
}
fn test_ssa_from_file_to_vtt_file() {
    let ssafile = ssa::parse("natsu.ass".to_string()).unwrap();
    ssafile
        .to_vtt()
        .to_file("natsu_1.vtt".to_string())
        .expect("Couldn't write");
}
fn test_ssa_from_file_to_ass_file() {
    let ssafile = ssa::parse("I'm Home-E27.ass".to_string()).unwrap();
    ssafile
        .to_file("I'm Home-E27_1.ass".to_string())
        .expect("Couldn't write");
}
fn test_srt_from_file_to_ass_file() {
    let ssafile = srt::parse("natsu.srt".to_string());
    ssafile
        .to_ass()
        .to_file("natsu_1.ass".to_string())
        .expect("Couldn't write");
}
fn test_srt_from_file_to_vtt_file() {
    let ssafile = srt::parse("natsu.srt".to_string());
    ssafile
        .to_vtt()
        .to_file("natsu_1_srt.vtt".to_string())
        .expect("Couldn't write");
}
fn test_ssa_from_file_to_default_file() {
    let ssafile = SSAFile::default();
    ssafile
        .to_file("default.ass".to_string())
        .expect("Couldn't write");
}
fn test_ssa_from_text_to_srt_file() {
    let file_value: &mut String = &mut "".to_string();
    File::open("I'm Home-E27.ass")
        .expect("WrongFile")
        .read_to_string(file_value)
        .expect("Couldn't write");
    let ssafile = ssa::parse(file_value.to_string()).unwrap();
    ssafile
        .to_srt()
        .to_file("I'm Home-E27_2.srt".to_string())
        .expect("Couldn't write");
}
fn test_srt_from_file_to_srt_file() {
    let srtfile = srt::parse("I'm Home-E27_1.srt".to_string());
    srtfile.to_file("I'm Home-E27_3.srt".to_string()).unwrap();
}
fn test_srt_from_text_to_srt_file() {
    let file_value: &mut String = &mut "".to_string();
    File::open("I'm Home-E27_1.srt")
        .expect("WrongFile")
        .read_to_string(file_value)
        .expect("Couldn't write");
    let srtfile = srt::parse(file_value.to_string());
    srtfile.to_file("I'm Home-E27_4.srt".to_string()).unwrap();
}
fn test_srt_from_text_to_srt_string() {
    let file_value: &mut String = &mut "".to_string();
    File::open("I'm Home-E27_1.srt")
        .expect("WrongFile")
        .read_to_string(file_value)
        .expect("Couldn't write");
    let file_value2: &mut String = &mut "".to_string();
    File::open("I'm Home-E27_2.srt")
        .expect("WrongFile")
        .read_to_string(file_value2)
        .expect("Couldn't write");
    let srtfile = srt::parse(file_value.to_string());
    assert_eq!(file_value2.to_string(), srtfile.stringify());
}
fn test_parse_vtt() {
    let file_value: &mut String = &mut "".to_string();
    File::open("test.vtt")
        .expect("WrongFile")
        .read_to_string(file_value)
        .expect("Couldn't write");
    let _vttfile: VTTFile = vtt::parse(file_value.to_owned());
}
fn test_parse_vtt_write_to_vtt() {
    let file_value: &mut String = &mut "".to_string();
    File::open("test.vtt")
        .expect("WrongFile")
        .read_to_string(file_value)
        .expect("Couldn't write");
    vtt::parse(file_value.to_owned())
        .to_file("vtt_to_file.vtt".to_string())
        .expect("Ok");
}
fn test_parse_vtt_write_to_ssa() {
    let file_value: &mut String = &mut "".to_string();
    File::open("natsu_1.vtt")
        .expect("WrongFile")
        .read_to_string(file_value)
        .expect("Couldn't write");
    vtt::parse(file_value.to_owned())
        .to_ass()
        .to_file("natsu_vtt_to_ass_file.ass".to_string())
        .expect("Ok");
}
fn test_parse_vtt_write_to_srt() {
    let file_value: &mut String = &mut "".to_string();
    File::open("test.vtt")
        .expect("WrongFile")
        .read_to_string(file_value)
        .expect("Couldn't write");
    vtt::parse(file_value.to_owned())
        .to_srt()
        .to_file("vtt_to_srt_file.srt".to_string())
        .expect("Ok");
}
fn main() {
    test_ssa_from_file_to_srt_file();
    test_ssa_from_file_to_default_file();
    test_ssa_from_file_to_ass_file();
    test_srt_from_file_to_ass_file();
    test_ssa_from_text_to_srt_file();
    test_srt_from_file_to_srt_file();
    test_srt_from_text_to_srt_file();
    test_srt_from_text_to_srt_string();
    test_parse_vtt();
    test_parse_vtt_write_to_vtt();
    test_parse_vtt_write_to_srt();
    test_srt_from_file_to_vtt_file();
    test_ssa_from_file_to_vtt_file();
    test_parse_vtt_write_to_ssa();
}
