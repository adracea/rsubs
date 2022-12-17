use rsubs_lib::{
    util::color::Color,
    util::time::{self, time_from_string},
};

fn main() {
    let b = Color::new(155, 213, 123, 0);
    println!("Hello, {b}!");
    let mut a = time_from_string(
        time::TIMESTAMP_SHORT
            .captures("2:00:01")
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .to_string(),
    );
    // let d = 12323;
    // let mut e = time::Time::new(
    //     String::from("2"),
    //     String::from("23"),
    //     String::from("34"),
    //     String::from("430"),
    //     Some(String::from("22796")),
    //     Some(String::from("23.75")),
    // );
    // a);
    println!("{:#}", a);
    for _i in 0..300 {
        a.sub_ms(50321)
            .unwrap_or_else(|_| a.sub_ms(a.total_ms()).unwrap());

        println!("{:#}", a);
        if a.total_ms() == 0 {
            break;
        }
    }
    // println!("{:#}", e);
    // e.add_ms(50321);
    // e.sub_ms(50321);
    // println!("{:#}", e);
}
