use std::ffi::CString;
use std::path::Path;

use std::os::unix::prelude::*;

use libc::{EINVAL,ENOMEM};

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
}
