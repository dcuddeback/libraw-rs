use std::ffi::CString;
use std::mem;
use std::path::Path;
use std::slice;

use std::os::unix::prelude::*;

use libc::{EINVAL,ENOMEM};

/// The data type for raw pixel data.
pub type RawPixel = u16;

/// The data type for raw 3-color pixel data.
pub type Color3Pixel = [u16; 3];

/// The data type for raw 4-color pixel data.
pub type Color4Pixel = [u16; 4];

/// A raw image.
pub struct Image {
    data: *mut ::libraw::libraw_data_t,
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            ::libraw::libraw_close(self.data);
        }
    }
}

impl Image {
    fn new() -> ::Result<Image> {
        let data = unsafe {
            ::libraw::libraw_init(::libraw::LIBRAW_OPIONS_NO_MEMERR_CALLBACK | ::libraw::LIBRAW_OPIONS_NO_DATAERR_CALLBACK)
        };

        if !data.is_null() {
            Ok(Image { data: data })
        }
        else {
            Err(::error::from_raw_os_error(ENOMEM))
        }
    }

    /// Opens the raw image file at the specified path.
    pub fn open(path: &Path) -> ::Result<Image> {
        let filename = match CString::new(path.as_os_str().as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(::error::from_raw_os_error(EINVAL))
        };

        let image = try!(Image::new());

        ::error::os::clear_errno();

        match unsafe { ::libraw::libraw_open_file(image.data, filename.as_ptr()) } {
            ::libraw::LIBRAW_SUCCESS => Ok(image),
            ::libraw::LIBRAW_IO_ERROR => {
                match ::error::os::errno() {
                    0 => Err(::error::from_libraw(::libraw::LIBRAW_IO_ERROR)),
                    errno => Err(::error::from_raw_os_error(errno)),
                }
            },
            err => Err(::error::from_libraw(err)),
        }
    }

    /// Unpacks the raw pixel data.
    ///
    /// The raw data must be unpacked before it can be accessed. After unpacking, the pixel type
    /// can be determined with `raw_pixel_type()`, and the pixel data can be accessed with
    /// `raw_pixmap()`, `color3_pixmap()`, or `color4_pixmap()`.
    pub fn unpack(&mut self) -> ::Result<()> {
        match unsafe { ::libraw::libraw_unpack(self.data) } {
            ::libraw::LIBRAW_SUCCESS => Ok(()),
            err => Err(::error::from_libraw(err)),
        }
    }

    /// Returns the pixel type of the raw data.
    ///
    /// The data must be unpacked before determining the pixel type. The value returned from this
    /// method determines which of the raw pixmap types is available.
    ///
    /// * `PixelType::Raw` means that `raw_pixmap()` is available.
    /// * `PixelType::Color3` means that `color3_pixmap()` is available.
    /// * `PixelType::Color4` means that `color4_pixmap()` is available.
    ///
    /// ## Errors
    ///
    /// This method returns an error if the pixel data has not been unpacked.
    pub fn raw_pixel_type(&self) -> ::Result<PixelType> {
        let rawdata = unsafe { &(*self.data).rawdata };

        if rawdata.raw_alloc.is_null() {
            return Err(::error::from_raw_os_error(EINVAL));
        }

        if !rawdata.raw_image.is_null() {
            return Ok(PixelType::Raw);
        }

        if !rawdata.color3_image.is_null() {
            return Ok(PixelType::Color3);
        }

        if !rawdata.color4_image.is_null() {
            return Ok(PixelType::Color4);
        }

        unreachable!();
    }

    /// Returns a pixmap of the raw pixels.
    ///
    /// ## Errors
    ///
    /// This method returns an error if the pixel data has not been unpacked or if the raw pixel
    /// data is in a different format (see `raw_pixel_type()`).
    pub fn raw_pixmap(&self) -> ::Result<Pixmap<RawPixel>> {
        let rawdata = unsafe { &(*self.data).rawdata };

        if !rawdata.raw_image.is_null() {
            let cols = rawdata.sizes.raw_width as usize;
            let rows = rawdata.sizes.raw_height as usize;

            Ok(Pixmap::new(rawdata.raw_image, cols, rows))
        }
        else {
            Err(::error::from_raw_os_error(EINVAL))
        }
    }

    /// Returns a pixmap of the raw 3-color pixels.
    ///
    /// ## Errors
    ///
    /// This method returns an error if the pixel data has not been unpacked or if the raw pixel
    /// data is in a different format (see `raw_pixel_type()`).
    pub fn color3_pixmap(&self) -> ::Result<Pixmap<Color3Pixel>> {
        let rawdata = unsafe { &(*self.data).rawdata };

        if !rawdata.raw_image.is_null() {
            let cols = rawdata.sizes.raw_width as usize;
            let rows = rawdata.sizes.raw_height as usize;

            Ok(Pixmap::new(rawdata.color3_image, cols, rows))
        }
        else {
            Err(::error::from_raw_os_error(EINVAL))
        }
    }

    /// Returns a pixmap of the raw 4-color pixels.
    ///
    /// ## Errors
    ///
    /// This method returns an error if the pixel data has not been unpacked or if the raw pixel
    /// data is in a different format (see `raw_pixel_type()`).
    pub fn color4_pixmap(&self) -> ::Result<Pixmap<Color4Pixel>> {
        let rawdata = unsafe { &(*self.data).rawdata };

        if !rawdata.raw_image.is_null() {
            let cols = rawdata.sizes.raw_width as usize;
            let rows = rawdata.sizes.raw_height as usize;

            Ok(Pixmap::new(rawdata.color4_image, cols, rows))
        }
        else {
            Err(::error::from_raw_os_error(EINVAL))
        }
    }
}

/// Types of raw pixel data.
pub enum PixelType {
    /// Each pixel is a single raw value, represented by the `RawPixel` type.
    Raw,

    /// Each pixel contains three color components, represented by the `Color3Pixel` type.
    Color3,

    /// Each pixel contains four color components, represented by the `Color4Pixel` type.
    Color4,
}

/// Maps pixel data onto a two-dimensional grid.
pub struct Pixmap<'a, T: Clone + 'static> {
    pixels: &'a [T],
    cols: usize,
    rows: usize,
}

impl<'a, T> Pixmap<'a, T> where T: Clone + 'static {
    fn new(ptr: *const T, cols: usize, rows: usize) -> Self {
        Pixmap {
            pixels: unsafe { slice::from_raw_parts(ptr, cols * rows) },
            cols: cols,
            rows: rows,
        }
    }

    /// Returns the number of columns.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns the number of rows.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of pixels.
    pub fn len(&self) -> usize {
        self.pixels.len()
    }

    /// Returns an iterator over the pixels.
    pub fn pixels(&'a self) -> Pixels<'a, T> {
        Pixels::new(self)
    }
}

/// Iterates over pixels in a pixmap.
pub struct Pixels<'a, T: Clone + 'static> {
    pixmap: &'a Pixmap<'a, T>,
    cur: *const T,
    end: *const T,
}

impl<'a, T> Pixels<'a, T> where T: Clone + 'static {
    fn new(pixmap: &'a Pixmap<'a, T>) -> Self {
        Pixels {
            pixmap: pixmap,
            end: unsafe { pixmap.pixels.as_ptr().offset(pixmap.pixels.len() as isize) },
            cur: pixmap.pixels.as_ptr(),
        }
    }
}

impl<'a, T> Iterator for Pixels<'a, T> where T: Clone + 'static {
    type Item = Pixel<'a, T>;

    fn next(&mut self) -> Option<Pixel<'a, T>> {
        if self.cur < self.end {
            let pixel = Pixel::new(self);

            self.cur = unsafe { self.cur.offset(1) };

            Some(pixel)
        }
        else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = unsafe {
            let cur: usize = mem::transmute(self.cur);
            let end: usize = mem::transmute(self.end);

            end - cur
        };

        (remaining, Some(remaining))
    }
}

/// A reference to a pixel in a pixmap.
pub struct Pixel<'a, T: Clone + 'static> {
    pixmap: &'a Pixmap<'a, T>,
    pixel: *const T,
}

impl<'a, T> Pixel<'a, T> where T: Clone + 'static {
    fn new(pixels: &Pixels<'a, T>) -> Self {
        Pixel {
            pixmap: pixels.pixmap,
            pixel: pixels.cur,
        }
    }

    /// Returns the column of the pixel's location within the pixmap.
    pub fn col(&self) -> usize {
        self.index() % self.pixmap.cols()
    }

    /// Returns the row of the pixel's location within the pixmap.
    pub fn row(&self) -> usize {
        self.index() / self.pixmap.cols()
    }

    /// Returns the pixel's value.
    pub fn value(&self) -> T {
        let pixel: &T = unsafe { mem::transmute(self.pixel) };
        pixel.clone()
    }

    fn index(&self) -> usize {
        unsafe {
            let pixel: usize = mem::transmute(self.pixel);
            let start: usize = mem::transmute(self.pixmap.pixels.as_ptr());

            (pixel - start) / mem::size_of::<T>()
        }
    }
}
