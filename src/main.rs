use std::{fs::File, io::Write};

use rsubs_lib::subs::ssa;

fn main() {
    let ssafile = ssa::parse("natsu.ass".to_string()).unwrap();
    let mut srtfile = File::options()
        .write(true)
        .create(true)
        .open("natsu.srt")
        .unwrap();
    srtfile.write_all(ssafile.to_srt().as_bytes()).unwrap();
}
