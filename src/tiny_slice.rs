use crate::Tagged;
use core::marker::PhantomData;
use core::ops::Deref;
use core::{fmt, slice};

/// `&[T]` but the size of a pointer.
pub struct TinySlice<'a, T> {
    tagged: Tagged<T>,
    _phantom: PhantomData<&'a [T]>,
}

impl<'a, T> TinySlice<'a, T> {
    #[inline]
    pub const fn from_slice(slice: &'a [T]) -> Self {
        let ptr = slice.as_ptr().as_mut();
        let len = slice.len();
        let tagged = unsafe { Tagged::new_unchecked(ptr).with_tag(len) };
        let _phantom = PhantomData;

        Self { tagged, _phantom }
    }

    #[inline]
    pub const fn as_slice(&self) -> &'a [T] {
        unsafe { slice::from_raw_parts(self.tagged.as_ptr(), self.tagged.tag()) }
    }
}

impl<'a, T> const Deref for TinySlice<'a, T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a, T> fmt::Debug for TinySlice<'a, T>
where
    T: fmt::Debug,
{
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.as_slice(), fmt)
    }
}
