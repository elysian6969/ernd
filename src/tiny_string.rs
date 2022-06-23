use crate::{Licity, Tagged};
use core::mem::ManuallyDrop;
use core::{fmt, mem, ptr, slice, str};

const DANGLING: *const u8 = ptr::invalid_mut(mem::align_of::<u8>());
const EMPTY: Tagged<u8> =
    unsafe { Tagged::new_unchecked(DANGLING.as_mut()).with_tag(Licity::EMPTY.get()) };

pub struct TinyString {
    tagged: Tagged<u8>,
}

impl TinyString {
    #[inline]
    pub const fn new() -> Self {
        let tagged = EMPTY;

        Self { tagged }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.tagged.as_ptr()
    }

    #[inline]
    pub fn to_raw_parts(&self) -> (*mut u8, usize, usize) {
        (self.tagged.as_ptr(), self.len(), self.capacity())
    }

    #[inline]
    pub unsafe fn from_raw_parts(ptr: *mut u8, len: usize, capacity: usize) -> Self {
        let licity = Licity::new_unchecked(len, capacity);
        let tagged = Tagged::new_unchecked(ptr).with_tag(licity.get());

        Self { tagged }
    }

    #[inline]
    unsafe fn string(&self) -> ManuallyDrop<String> {
        let (ptr, len, capacity) = self.to_raw_parts();
        let string = ManuallyDrop::new(String::from_raw_parts(ptr, len, capacity));

        string
    }

    #[inline]
    unsafe fn update(&mut self, string: ManuallyDrop<String>) {
        let (ptr, len, capacity) = ManuallyDrop::into_inner(string).into_raw_parts();

        *self = Self::from_raw_parts(ptr, len, capacity);
    }

    #[inline]
    pub fn push_str(&mut self, string: &str) {
        let mut inner = unsafe { self.string() };

        inner.push_str(string);

        unsafe {
            self.update(inner);
        }
    }

    #[inline]
    pub fn licity(&self) -> Licity {
        unsafe { Licity::from_raw(self.tagged.tag()) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.licity().len()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.licity().capacity()
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe {
            let bytes = slice::from_raw_parts(self.as_ptr(), self.len());
            let string = str::from_utf8_unchecked(bytes);

            string
        }
    }
}

impl fmt::Debug for TinyString {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.as_str(), fmt)
    }
}
