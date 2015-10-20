use std::ffi::CStr;
use std::slice;
use std::str;

use libc::{c_char};

/// Iterator over a list of supported cameras.
pub struct Cameras {
    iter: slice::Iter<'static, *const c_char>,
}

impl Iterator for Cameras {
    type Item = &'static str;

    fn next(&mut self) -> Option<&'static str> {
        self.iter.next().map(|ptr| {
            unsafe {
                str::from_utf8_unchecked(CStr::from_ptr(*ptr).to_bytes())
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

/// Returns a list of the supported cameras.
pub fn camera_list() -> Cameras {
    unsafe {
        let list = ::libraw::libraw_cameraList();
        let len = ::libraw::libraw_cameraCount();

        Cameras {
            iter: slice::from_raw_parts(list, len as usize).iter()
        }
    }
}
