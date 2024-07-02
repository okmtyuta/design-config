use generators::{configs::configs::Configs, entities::color::RGB};
use std::env;

fn main() {
    // let current_dir = env::current_dir().expect("Cannot get generator dir.");
    // let path = current_dir.join("src").join("config.json");

    // let configs = Configs::from_file(path);

    // println!("{:?}", configs)

    let rgb = RGB::from_rgb((210, 120, 20));
    let hsv = rgb.to_hsv();
    let r = hsv.to_rgb();

    println!("{:?}", hsv);

    println!("{:?}", rgb);
    println!("{:?}", r);
}
