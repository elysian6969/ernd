use crate::TinySlice;
use core::ops::Deref;
use core::{fmt, str};

/// `&str` but the size of a pointer.
pub struct TinyStr<'a> {
    bytes: TinySlice<'a, u8>,
}

impl<'a> TinyStr<'a> {
    #[inline]
    pub const fn from_str(slice: &'a str) -> Self {
        let bytes = TinySlice::from_slice(slice.as_bytes());

        Self { bytes }
    }

    #[inline]
    pub const fn as_str(&self) -> &'a str {
        unsafe { str::from_utf8_unchecked(self.bytes.as_slice()) }
    }
}

impl<'a> const Deref for TinyStr<'a> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<'a> fmt::Debug for TinyStr<'a> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.as_str(), fmt)
    }
}
