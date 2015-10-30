extern crate libraw;

use std::path::Path;

#[test]
fn it_can_read_raw_pixel_data() {
    let mut image = libraw::Image::open(Path::new("tests/data/RAW_NIKON_D1.NEF")).unwrap();

    image.unpack().unwrap();

    let raw = image.raw_pixmap().unwrap();

    let sum = raw.pixels().fold(0, |accum, pixel| {
        accum + pixel.value() as usize
    });

    assert_eq!(1261062932, sum);
}
