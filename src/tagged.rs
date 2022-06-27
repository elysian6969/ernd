use crate::consts::{PTR_BITS, PTR_MASK};
use core::{fmt, mem};

/// `*mut T` but tagged.
#[repr(transparent)]
pub struct Tagged<T> {
    pointer: *const T,
}

impl<T> Tagged<T> {
    #[inline]
    pub const unsafe fn new_unchecked(ptr: *mut T) -> Self {
        Self {
            pointer: ptr.as_const(),
        }
    }

    #[inline]
    pub const fn new(ptr: *mut T) -> Option<Self> {
        let this = unsafe { Self::new_unchecked(ptr) };
        let (_ptr, tag) = this.to_parts();

        if tag == 0 {
            Some(this)
        } else {
            None
        }
    }

    /// Construct a tagged pointer from raw parts.
    #[inline]
    pub const unsafe fn from_parts(ptr: *mut T, tag: u32) -> Self {
        let addr = expose_addr(ptr);
        let ptr = from_exposed_addr_mut(addr | ((tag as usize) << PTR_BITS));

        Self::new_unchecked(ptr)
    }

    /// Decompose this tagged pointer into raw parts.
    #[inline]
    pub const fn to_parts(self) -> (*mut T, u32) {
        let addr = expose_addr(self.pointer.as_mut());
        let ptr = from_exposed_addr_mut(addr & PTR_MASK);
        let tag = addr >> PTR_BITS;

        (ptr, tag as u32)
    }

    #[inline]
    pub const fn addr(self) -> usize {
        expose_addr(self.as_ptr())
    }

    #[inline]
    pub const fn with_addr(self, addr: usize) -> Self {
        let (ptr, tag) = self.to_parts();
        let ptr = with_addr(ptr, addr);

        unsafe { Self::from_parts(ptr, tag) }
    }

    #[inline]
    pub const fn as_ptr(self) -> *mut T {
        self.to_parts().0
    }

    #[inline]
    pub const fn tag(self) -> u32 {
        self.to_parts().1
    }

    #[inline]
    pub const fn with_tag(self, tag: u32) -> Self {
        let (ptr, _tag) = self.to_parts();

        unsafe { Self::from_parts(ptr, tag) }
    }
}

impl<T> const Clone for Tagged<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Tagged<T> {}

impl<T> fmt::Debug for Tagged<T> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.as_ptr(), fmt)
    }
}

#[inline]
const fn addr<T>(ptr: *mut T) -> usize {
    // FIXME: use `addr` when it is `const fn`
    unsafe { mem::transmute(ptr) }
}

#[inline]
const fn expose_addr<T>(ptr: *mut T) -> usize {
    // FIXME: use `pointer::expose_addr` when it is `const fn`
    unsafe { mem::transmute(ptr) }
}

#[inline]
const fn with_addr<T>(ptr: *mut T, addr: usize) -> *mut T {
    // FIXME: use `pointer::with_addr` when it is `const fn`
    let self_addr = self::addr(ptr) as isize;
    let dest_addr = addr as isize;
    let offset = dest_addr.wrapping_sub(self_addr);

    unsafe { ptr.byte_offset(offset) }
}

#[inline]
const fn from_exposed_addr_mut<T>(addr: usize) -> *mut T {
    // FIXME: use `core::ptr::from_exposed_addr_mut` when it is `const fn`
    addr as *mut T
}
