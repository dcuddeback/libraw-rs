extern crate libraw;

fn main() {
    println!("libraw {}", libraw::version());
    println!("");

    for camera in libraw::camera_list() {
        println!("{}", camera);
    }
}
