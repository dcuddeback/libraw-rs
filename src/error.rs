use std::error::Error as StdError;
use std::ffi::CStr;
use std::fmt;
use std::result::Result as StdResult;

use libc::{c_int};

/// Result type returned by libraw functions.
pub type Result<T> = StdResult<T, Error>;

/// The error type for libraw functions.
#[derive(Debug)]
pub struct Error {
    repr: Repr,
    message: String,
}

#[derive(Debug)]
enum Repr {
    LibRaw(c_int),
    Os(i32),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> StdResult<(), fmt::Error> {
        fmt.write_str(&self.message)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

#[doc(hidden)]
pub fn from_libraw(error: c_int) -> Error {
    let message = String::from_utf8_lossy(unsafe {
        CStr::from_ptr(::libraw::libraw_strerror(error))
    }.to_bytes()).into_owned();

    Error {
        repr: Repr::LibRaw(error),
        message: message,
    }
}

#[doc(hidden)]
pub fn from_raw_os_error(errno: i32) -> Error {
    Error {
        repr: Repr::Os(errno),
        message: os::error_string(errno),
    }
}


// adapted from libstd
pub mod os {
    use std::ffi::CStr;
    use std::str;

    use libc::{c_char,c_int,size_t};

    const TMPBUF_SZ: usize = 128;

    #[cfg(any(target_os = "macos",
              target_os = "ios",
              target_os = "freebsd"))]
    unsafe fn errno_location() -> *mut c_int {
        extern { fn __error() -> *mut c_int; }
        __error()
    }

    #[cfg(target_os = "bitrig")]
    fn errno_location() -> *mut c_int {
        extern {
            fn __errno() -> *mut c_int;
        }
        unsafe {
            __errno()
        }
    }

    #[cfg(target_os = "dragonfly")]
    unsafe fn errno_location() -> *mut c_int {
        extern { fn __dfly_error() -> *mut c_int; }
        __dfly_error()
    }

    #[cfg(target_os = "openbsd")]
    unsafe fn errno_location() -> *mut c_int {
        extern { fn __errno() -> *mut c_int; }
        __errno()
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    unsafe fn errno_location() -> *mut c_int {
        extern { fn __errno_location() -> *mut c_int; }
        __errno_location()
    }

    pub fn errno() -> i32 {
        unsafe {
            (*errno_location()) as i32
        }
    }

    pub fn clear_errno() {
        unsafe {
            (*errno_location()) = 0;
        }
    }

    pub fn error_string(errno: i32) -> String {
        #[cfg(target_os = "linux")]
        extern {
            #[link_name = "__xpg_strerror_r"]
            fn strerror_r(errnum: c_int, buf: *mut c_char,
                          buflen: size_t) -> c_int;
        }
        #[cfg(not(target_os = "linux"))]
        extern {
            fn strerror_r(errnum: c_int, buf: *mut c_char,
                          buflen: size_t) -> c_int;
        }

        let mut buf = [0 as c_char; TMPBUF_SZ];

        let p = buf.as_mut_ptr();
        unsafe {
            if strerror_r(errno as c_int, p, buf.len() as size_t) < 0 {
                panic!("strerror_r failure");
            }

            let p = p as *const _;
            str::from_utf8(CStr::from_ptr(p).to_bytes()).unwrap().to_string()
        }
    }
}
