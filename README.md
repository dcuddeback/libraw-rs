# LibRaw

The `libraw` crate provides a safe wrapper around the native `libraw` library.

## Dependencies
In order to use the `libraw` crate, you must have the `libraw_r` library installed where it can be
found by `pkg-config`. `libraw_r` is the reentrant version of LibRaw. Linking against the non
reentrant `libraw` is not supported.

On Debian-based Linux distributions, install the `libraw-dev` package:

```
sudo apt-get install libraw-dev
```

On OS X, install `libraw` with Homebrew:

```
brew install libraw
```

On FreeBSD, install the `libraw` package:

```
sudo pkg install libraw
```

## Usage
Add `libraw` as a dependency in `Cargo.toml`:

```toml
[dependencies]
libraw = "0.1"
```

Import the `libraw` crate. Open an image with `Image::open()` and then use the methods on `Image` to
operate on and inspect the raw image data.

```rust
extern crate libraw;

use std::path::Path;

fn main() {
    let mut image = libraw::Image::open(Path::new("image.nef")).unwrap();

    image.unpack().unwrap();
    let raw = image.raw_pixmap().unwrap();

    let sum = raw.pixels().fold(0, |accum, pixel| {
        accum + pixel.value() as usize
    });

    println!("average pixel brightness = {:.3}", sum as f64 / raw.len() as f64);
}
```

## License
Copyright © 2015 David Cuddeback

Distributed under the [MIT License](LICENSE).

*Note:* By using this crate, your executable will link to the `libraw` C library, which is available
under the [LGPL version 2.1, CDDL version 1.0, or LibRaw Software
License](https://github.com/LibRaw/LibRaw/blob/master/COPYRIGHT).
