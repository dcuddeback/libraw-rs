use std::fmt;

/// LibRaw library version.
#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
pub struct Version {
    inner: u32,
}

impl Version {
    /// Creates a new version with the given major, minor, and patch numbers.
    pub fn new(major: u8, minor: u8, patch: u8) -> Version {
        Version {
            inner: (major as u32) << 16 | (minor as u32) << 8 | (patch as u32) << 0
        }
    }

    /// Returns the major version number.
    pub fn major(&self) -> u8 {
        ((self.inner & 0xFF0000) >> 16) as u8
    }

    /// Returns the minor version number.
    pub fn minor(&self) -> u8 {
        ((self.inner & 0x00FF00) >> 8) as u8
    }

    /// Returns the patch version number.
    pub fn patch(&self) -> u8 {
        ((self.inner & 0x0000FF) >> 0) as u8
    }
}

impl fmt::Display for Version {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!("{}.{}.{}", self.major(), self.minor(), self.patch()))
    }
}

/// Returns the version of the LibRaw library.
pub fn version() -> Version {
    Version {
        inner: unsafe { ::libraw::libraw_versionNumber() as u32 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_stores_major_version() {
        assert_eq!(1, Version::new(1, 0, 0).major());
        assert_eq!(2, Version::new(2, 0, 0).major());
    }

    #[test]
    fn it_stores_minor_version() {
        assert_eq!(1,   Version::new(0, 1,   0).minor());
        assert_eq!(2,   Version::new(0, 2,   0).minor());
        assert_eq!(255, Version::new(0, 255, 0).minor());
    }

    #[test]
    fn it_stores_patch_version() {
        assert_eq!(1,   Version::new(0, 0, 1).patch());
        assert_eq!(2,   Version::new(0, 0, 2).patch());
        assert_eq!(255, Version::new(0, 0, 255).patch());
    }

    #[test]
    fn it_renders_version_string() {
        assert_eq!(String::from("1.2.3"), format!("{}", Version::new(1, 2, 3)));
    }
}
