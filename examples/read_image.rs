extern crate libraw;
extern crate libraw_sys;

use std::env;
use std::path::Path;

fn main() {
    for arg in env::args_os().skip(1) {
        println!("opening {:?}", arg);

        let mut image = libraw::Image::open(Path::new(&arg)).unwrap();

        println!("unpacking ...");
        image.unpack().unwrap();
        println!(" (done)");

        let raw = image.raw_pixmap().unwrap();

        let mut sum: usize = 0;

        for pixel in raw.pixels() {
            sum += pixel.value() as usize;
        }

        println!("total pixel brightness = {}", sum);
    }
}
